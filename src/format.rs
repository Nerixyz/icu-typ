use std::str::FromStr;

use icu_datetime::{
    fieldsets::{builder::FieldSetBuilder, enums::CompositeFieldSet},
    DateTimeFormatter,
};
use icu_locale_core::Locale;
use icu_time::{
    zone::{
        iana::IanaParserExtendedBorrowed, TimeZoneVariant, VariantOffsetsCalculator,
        ZoneNameTimestamp,
    },
    DateTime, TimeZone, TimeZoneInfo, ZonedDateTime,
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

pub struct SpecifiedZonedDateTime {
    pub value:
        icu_time::ZonedDateTime<icu_calendar::Iso, TimeZoneInfo<icu_time::zone::models::Full>>,
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
            (false, icu_time::Time::start_of_day())
        };

        let (has_zone, zone) = if let Some(spec) = value.zone {
            let bcp47 = match (spec.bcp47, spec.iana) {
                (None, None) => TimeZone::UNKNOWN,
                (Some(id), None) => TimeZone(id.parse().map_err(Self::Error::IcuLocaleParse)?),
                (None, Some(spec)) => {
                    let parsed = IanaParserExtendedBorrowed::new().parse(&spec);
                    if parsed.time_zone.is_unknown() {
                        return Err(Self::Error::UnknownIana);
                    }
                    parsed.time_zone
                }
                (Some(_), Some(_)) => return Err(Self::Error::IanaAndBcp47),
            };
            // XXX: we need to keep track of the date before we use this time zone
            let tz = bcp47
                .with_offset(spec.offset.map(TryInto::try_into).transpose()?)
                .at_date_time_iso(DateTime { date, time });

            let tz = if let Some(offset) = tz.offset() {
                // Only resolve the zone variant if a date was specified.
                // Otherwise we'd be at 1970-01-01 which would be confusing.
                let offsets = if has_date {
                    VariantOffsetsCalculator::new()
                        .compute_offsets_from_time_zone_and_name_timestamp(
                            tz.id(),
                            ZoneNameTimestamp::from_date_time_iso(DateTime { date, time }),
                        )
                } else {
                    None
                };
                tz.with_variant(match offsets {
                    Some(offsets) => {
                        if offsets.standard == offset {
                            TimeZoneVariant::Standard
                        } else if offsets.daylight == Some(offset) {
                            TimeZoneVariant::Daylight
                        } else {
                            return Err(Self::Error::OffsetMismatch(crate::InvalidVariantOffsets(
                                offsets,
                            )));
                        }
                    }
                    None => TimeZoneVariant::Standard,
                })
            } else {
                tz.with_variant(TimeZoneVariant::Standard)
            };

            (true, tz)
        } else {
            (
                false,
                TimeZoneInfo::utc()
                    .at_date_time_iso(DateTime { date, time })
                    .with_variant(TimeZoneVariant::Standard),
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
