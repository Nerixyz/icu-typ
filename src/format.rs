use std::str::FromStr;

use icu_datetime::{
    fieldsets::{builder::FieldSetBuilder, enums::CompositeFieldSet},
    DateTimeFormatter,
};
use icu_locale_core::Locale;
use icu_time::{
    zone::{iana::IanaParserExtendedBorrowed, TimeZoneVariant, UtcOffsetCalculator},
    TimeZone, TimeZoneInfo, ZonedDateTime,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Spec {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub nanosecond: Option<u32>,
    pub zone: Option<TimezoneSpec>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TimezoneSpec {
    offset: Option<UtcOffsetSpec>,
    iana: Option<String>,
    bcp47: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum UtcOffsetSpec {
    OffsetSeconds(i32),
    Chars(String),
}

pub fn format(spec: Spec, locale: &str, builder: FieldSetBuilder) -> Result<Vec<u8>, crate::Error> {
    let locale = Locale::from_str(locale)?;
    let fields = builder.build_composite()?;

    let spec: SpecifiedZonedDateTime = spec.try_into()?;
    if !spec.check_fieldset(&fields) {
        return Err(crate::Error::MissingValues);
    }

    let formatter = DateTimeFormatter::try_new(locale.into(), fields)
        .map_err(crate::Error::LoadDateTimeFormatter)?;

    Ok(crate::write::to_vec(formatter.format(&spec.value)))
}

struct SpecifiedZonedDateTime {
    value: icu_time::ZonedDateTime<icu_calendar::Iso, TimeZoneInfo<icu_time::zone::models::Full>>,
    has_date: bool,
    has_time: bool,
    has_zone: bool,
}

impl SpecifiedZonedDateTime {
    pub fn check_fieldset(&self, set: &CompositeFieldSet) -> bool {
        match set {
            CompositeFieldSet::Date(_) => self.has_date,
            CompositeFieldSet::CalendarPeriod(_) => self.has_date,
            CompositeFieldSet::Time(_) => self.has_time,
            CompositeFieldSet::Zone(_) => self.has_zone,
            CompositeFieldSet::DateTime(_) => self.has_date && self.has_time,
            CompositeFieldSet::DateZone(_) => self.has_date && self.has_zone,
            CompositeFieldSet::TimeZone(_) => self.has_time && self.has_zone,
            CompositeFieldSet::DateTimeZone(_) => self.has_date && self.has_time && self.has_zone,
            _ => {
                debug_assert!(false, "Unknown fieldset");
                true
            }
        }
    }
}

impl TryFrom<Spec> for SpecifiedZonedDateTime {
    type Error = crate::Error;

    fn try_from(value: Spec) -> Result<Self, Self::Error> {
        let (has_date, date) = match (value.year, value.month, value.day) {
            (Some(y), Some(m), Some(d)) => (
                true,
                icu_calendar::Date::try_new_iso(y, m, d).map_err(Self::Error::DateRange)?,
            ),
            (None, None, None) => (
                false,
                icu_calendar::Date::try_new_iso(1970, 1, 1).map_err(Self::Error::DateRange)?,
            ),
            _ => return Err(Self::Error::PartialDate),
        };

        let (has_time, time) = if let Some(hour) = value.hour {
            (
                true,
                icu_time::Time::new(
                    hour.try_into().map_err(Self::Error::DateRange)?,
                    value
                        .minute
                        .unwrap_or_default()
                        .try_into()
                        .map_err(Self::Error::DateRange)?,
                    value
                        .second
                        .unwrap_or_default()
                        .try_into()
                        .map_err(Self::Error::DateRange)?,
                    value
                        .nanosecond
                        .unwrap_or_default()
                        .try_into()
                        .map_err(Self::Error::DateRange)?,
                ),
            )
        } else {
            if value.minute.is_some() || value.second.is_some() || value.nanosecond.is_some() {
                return Err(Self::Error::PartialTime);
            }
            (false, icu_time::Time::midnight())
        };

        let (has_zone, zone) = if let Some(spec) = value.zone {
            let bcp47 = match (spec.bcp47, spec.iana) {
                (None, None) => TimeZone::unknown(),
                (Some(id), None) => TimeZone(id.parse().map_err(|e| Self::Error::TinyStr(e, id))?),
                (None, Some(spec)) => {
                    let (tz, _canonical, _normalized) =
                        IanaParserExtendedBorrowed::new().parse(&spec);
                    if tz == TimeZone::unknown() {
                        return Err(Self::Error::UnknownIana);
                    }
                    tz
                }
                (Some(_), Some(_)) => return Err(Self::Error::IanaAndBcp47),
            };
            // XXX: we need to keep track of the date before we use this time zone
            let tz = bcp47
                .with_offset(spec.offset.map(TryInto::try_into).transpose()?)
                .at_time((date, time));

            let tz = if let Some(offset) = tz.offset() {
                let offsets = UtcOffsetCalculator::new()
                    .compute_offsets_from_time_zone(tz.time_zone_id(), (date, time));
                tz.with_zone_variant(match offsets {
                    Some(offsets) => {
                        if offsets.standard == offset {
                            TimeZoneVariant::Standard
                        } else if offsets.daylight == Some(offset) {
                            TimeZoneVariant::Daylight
                        } else {
                            return Err(Self::Error::OffsetMismatch(crate::InvalidUtcOffsets(
                                offsets,
                            )));
                        }
                    }
                    None => TimeZoneVariant::Standard,
                })
            } else {
                tz.with_zone_variant(TimeZoneVariant::Standard)
            };

            (true, tz)
        } else {
            (
                false,
                TimeZoneInfo::utc()
                    .at_time((date, time))
                    .with_zone_variant(TimeZoneVariant::Standard),
            )
        };

        Ok(Self {
            value: ZonedDateTime { date, time, zone },
            has_date,
            has_time,
            has_zone,
        })
    }
}

impl TryFrom<UtcOffsetSpec> for icu_time::zone::UtcOffset {
    type Error = crate::Error;

    fn try_from(value: UtcOffsetSpec) -> Result<Self, Self::Error> {
        match value {
            UtcOffsetSpec::OffsetSeconds(sec) => Self::try_from_seconds(sec),
            UtcOffsetSpec::Chars(chars) => Self::try_from_str(&chars),
        }
        .map_err(|_| Self::Error::InvalidOffset)
    }
}

#[cfg(test)]
mod tests {
    use icu_calendar::{Date, Iso};
    use icu_datetime::{
        fieldsets::{
            builder::{DateFields, FieldSetBuilder},
            enums::{CompositeFieldSet, TimeFieldSet, ZoneFieldSet},
            zone, T,
        },
        DateTimeFormatter, NoCalendarFormatter,
    };
    use icu_locale_core::locale;
    use icu_time::{
        zone::{IanaParser, TimeZoneVariant, UtcOffsetCalculator},
        Time, TimeZone, ZonedDateTime,
    };

    use super::{Spec, TimezoneSpec};

    #[test]
    fn basic() {
        let zdt = ZonedDateTime::try_from_str(
            "2024-08-08T12:08:19+01:00[Europe/Berlin]",
            Iso,
            IanaParser::new(),
            &UtcOffsetCalculator::new(),
        )
        .unwrap();

        let formatter = DateTimeFormatter::try_new(
            locale!("de").into(),
            CompositeFieldSet::TimeZone(
                TimeFieldSet::T(T::long()).zone(ZoneFieldSet::GenericLong(zone::GenericLong)),
            ),
        )
        .unwrap();
        dbg!(formatter.format(&zdt).to_string()); // 12:08:19 MEZ

        let formatter =
            DateTimeFormatter::try_new(locale!("de").into(), T::long().zone(zone::GenericLong))
                .unwrap();
        dbg!(formatter.format(&zdt).to_string()); // 12:08:19 MEZ

        let formatter =
            DateTimeFormatter::try_new(locale!("de").into(), zone::GenericLong).unwrap();
        dbg!(formatter.format(&zdt).to_string()); // Mitteleuropäische Normalzeit
    }

    #[test]
    fn other() {
        let spec = Spec {
            year: Some(2024),
            month: Some(7),
            day: Some(3),
            hour: Some(6),
            minute: Some(12),
            second: Some(4),
            nanosecond: None,
            zone: Some(TimezoneSpec {
                offset: Some(super::UtcOffsetSpec::Chars("-05".into())),
                bcp47: Some("uschi".into()),
                iana: None,
            }),
        };
        let mut builder = FieldSetBuilder::new();
        builder.date_fields = Some(DateFields::MD);
        builder.time_precision = Some(icu_datetime::options::TimePrecision::Hour);
        builder.zone_style = Some(icu_datetime::fieldsets::builder::ZoneStyle::SpecificLong);
        dbg!(String::from_utf8(format::format(spec, "en", builder).unwrap()).unwrap());
    }

    #[test]
    fn basic2() {
        let zdt = ZonedDateTime::try_from_str(
            "2024-08-08T12:08:19+01:00[Europe/Berlin]",
            Iso,
            IanaParser::new(),
            &UtcOffsetCalculator::new(),
        )
        .unwrap();

        let formatter =
            DateTimeFormatter::try_new(locale!("en").into(), zone::GenericShort).unwrap();
        dbg!(formatter.format(&zdt).to_string()); // 12:08:19 MEZ
    }

    #[test]
    fn basic3() {
        let fmt = NoCalendarFormatter::try_new(locale!("en").into(), zone::GenericShort).unwrap();

        // Time zone info for America/Chicago in the summer
        let zone = TimeZone(tinystr::tinystr!(8, "debsngn"))
            .with_offset("-05".parse().ok())
            .at_time((Date::try_new_iso(2022, 8, 29).unwrap(), Time::midnight()))
            .with_zone_variant(TimeZoneVariant::Daylight);
        dbg!(fmt.format(&zone).to_string());
    }
}
