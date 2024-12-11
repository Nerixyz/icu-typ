use std::str::FromStr;

use icu_datetime::{
    fieldsets::{enums::CompositeFieldSet, serde::CompositeFieldSetSerde},
    DateTimeFormatter,
};
use icu_locale_core::Locale;
use icu_timezone::{
    models::Full, CustomZonedDateTime, TimeZoneBcp47Id, TimeZoneIdMapperWithFastCanonicalization,
    TimeZoneInfo, ZoneOffsetCalculator, ZoneVariant,
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
    bcp47: Option<TimeZoneBcp47Id>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum UtcOffsetSpec {
    OffsetSeconds(i32),
    Chars(String),
}

#[derive(Deserialize)]
pub struct FormatOptions {
    pub locale: String,
    pub composite: CompositeFieldSetSerde,
}

pub fn format(spec: Spec, formatter_opts: FormatOptions) -> Result<Vec<u8>, crate::Error> {
    let locale = Locale::from_str(&formatter_opts.locale)?;
    let fields: CompositeFieldSet = formatter_opts
        .composite
        .try_into()
        .map_err(crate::Error::CompositeError)?;

    let spec: SpecifiedZonedDateTime = spec.try_into()?;
    if !spec.check_fieldset(&fields) {
        return Err(crate::Error::MissingValues);
    }

    let formatter = DateTimeFormatter::try_new(locale.into(), fields)
        .map_err(crate::Error::LoadDateTimeFormatter)?;

    Ok(crate::write::to_vec(
        formatter.format_any_calendar(&spec.value),
    ))
}

struct SpecifiedZonedDateTime {
    value: icu_timezone::CustomZonedDateTime<icu_calendar::Iso, TimeZoneInfo<Full>>,
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
            CompositeFieldSet::DateZone(_, _) => self.has_date && self.has_zone,
            CompositeFieldSet::TimeZone(_, _) => self.has_time && self.has_zone,
            CompositeFieldSet::DateTimeZone(_, _) => {
                self.has_date && self.has_time && self.has_zone
            }
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
                icu_calendar::Date::try_new_iso(1970, 0, 0).map_err(Self::Error::DateRange)?,
            ),
            _ => return Err(Self::Error::PartialDate),
        };

        let (has_time, time) = if let Some(hour) = value.hour {
            (
                true,
                icu_calendar::Time::new(
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
            (false, icu_calendar::Time::midnight())
        };

        let (has_zone, zone) = if let Some(spec) = value.zone {
            let bcp47 = match (spec.bcp47, spec.iana) {
                (None, None) => TimeZoneBcp47Id::unknown(),
                (Some(id), None) => id,
                (None, Some(spec)) => {
                    let bcp = TimeZoneIdMapperWithFastCanonicalization::new()
                        .inner()
                        .iana_to_bcp47(&spec);
                    if bcp == TimeZoneBcp47Id::unknown() {
                        return Err(Self::Error::UnknownIana);
                    }
                    bcp
                }
                (Some(_), Some(_)) => return Err(Self::Error::IanaAndBcp47),
            };
            // XXX: we need to keep track of the date before we use this time zone
            let tz = bcp47
                .with_offset(spec.offset.map(TryInto::try_into).transpose()?)
                .at_time((date, time));

            let tz = if let Some(offset) = tz.offset() {
                let offsets = ZoneOffsetCalculator::new()
                    .compute_offsets_from_time_zone(tz.time_zone_id(), (date, time));
                tz.with_zone_variant(match offsets {
                    Some(offsets) => {
                        if offsets.standard == offset {
                            ZoneVariant::Standard
                        } else if offsets.daylight == Some(offset) {
                            ZoneVariant::Daylight
                        } else {
                            return Err(Self::Error::OffsetMismatch);
                        }
                    }
                    None => icu_timezone::ZoneVariant::Standard,
                })
            } else {
                tz.with_zone_variant(icu_timezone::ZoneVariant::Standard)
            };

            (true, tz)
        } else {
            (
                false,
                TimeZoneInfo::utc()
                    .at_time((date, time))
                    .with_zone_variant(ZoneVariant::Standard),
            )
        };

        Ok(Self {
            value: CustomZonedDateTime { date, time, zone },
            has_date,
            has_time,
            has_zone,
        })
    }
}

impl TryFrom<UtcOffsetSpec> for icu_timezone::UtcOffset {
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
    use icu_datetime::{
        fieldsets::{
            enums::{CompositeFieldSet, TimeFieldSet, ZoneStyle},
            T, Z,
        },
        DateTimeFormatter,
    };
    use icu_locale_core::locale;
    use icu_timezone::IxdtfParser;

    #[test]
    fn basic() {
        let zdt = IxdtfParser::new()
            .try_from_str("2024-08-08T12:08:19+01:00[Europe/Berlin]")
            .unwrap();

        let formatter = DateTimeFormatter::try_new(
            locale!("de").into(),
            CompositeFieldSet::TimeZone(TimeFieldSet::T(T::long()), ZoneStyle::Z),
        )
        .unwrap();
        dbg!(formatter.format_any_calendar(&zdt).to_string()); // 12:08:19 MEZ

        let formatter =
            DateTimeFormatter::try_new(locale!("de").into(), T::long().zone_z()).unwrap();
        dbg!(formatter.format_any_calendar(&zdt).to_string()); // 12:08:19 MEZ

        let formatter = DateTimeFormatter::try_new(locale!("de").into(), Z::long()).unwrap();
        dbg!(formatter.format_any_calendar(&zdt).to_string()); // Mitteleuropäische Normalzeit
    }
}
