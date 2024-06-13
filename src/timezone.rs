use std::str::FromStr;

use icu_datetime::time_zone::TimeZoneFormatter;
use icu_locid::Locale;
use icu_timezone::{CustomTimeZone, MetazoneCalculator, TimeZoneIdMapper};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Spec {
    offset: GmtOffsetSpec,
    timezone_id: Option<TimezoneIdSpec>,
    metazone: Option<MetazoneSpec>,
    zone_variant: Option<icu_timezone::ZoneVariant>,
}

#[derive(Deserialize)]
pub struct FormatOptions {
    pub locale: String,
    pub fallback: FallbackSpec,
    pub format: Option<IncludeSpec>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum GmtOffsetSpec {
    OffsetSeconds(i32),
    Chars(String),
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MetazoneSpec {
    Id(icu_timezone::MetazoneId),
    LocalDate(crate::datetime::Spec),
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimezoneIdSpec {
    Bcp47(String),
    Iana(String),
}

#[derive(Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IsoFormatSpec {
    Basic,
    Extended,
    UtcBasic,
    UtcExtended,
}

#[derive(Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IsoMinutesSpec {
    Required,
    Optional,
}

#[derive(Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IsoSecondsSpec {
    Never,
    Optional,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FallbackSpec {
    Iso8601 {
        format: IsoFormatSpec,
        minutes: IsoMinutesSpec,
        seconds: IsoSecondsSpec,
    },
    LocalizedGmt,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IncludeSpec {
    Iso8601 {
        format: IsoFormatSpec,
        minutes: IsoMinutesSpec,
        seconds: IsoSecondsSpec,
    },
    GenericLocation,
    GenericNonLocationLong,
    GenericNonLocationShort,
    LocalizedGmt,
    SpecificNonLocationLong,
    SpecificNonLocationShort,
}

pub fn format(tz: CustomTimeZone, formatter_opts: FormatOptions) -> Result<Vec<u8>, crate::Error> {
    let locale = Locale::from_str(&formatter_opts.locale)?;
    let mut formatter = TimeZoneFormatter::try_new(&locale.into(), formatter_opts.fallback.into())?;
    if let Some(spec) = formatter_opts.format {
        spec.apply(&mut formatter)?;
    }

    Ok(crate::write::to_vec(formatter.format(&tz)))
}

impl TryFrom<Spec> for CustomTimeZone {
    type Error = crate::Error;

    fn try_from(value: Spec) -> Result<Self, Self::Error> {
        let mut base = Self {
            gmt_offset: Some(value.offset.try_into()?),
            time_zone_id: None,
            metazone_id: None,
            zone_variant: value.zone_variant,
        };

        match value.timezone_id {
            Some(TimezoneIdSpec::Bcp47(bcp)) => {
                base.time_zone_id = Some(
                    bcp.as_str()
                        .parse()
                        .map_err(|e| Self::Error::TinyStr(e, bcp))?,
                )
            }
            Some(TimezoneIdSpec::Iana(iana)) => {
                let mapper = TimeZoneIdMapper::new();
                let mapped = mapper.as_borrowed().iana_to_bcp47(&iana);
                if mapped.is_none() {
                    return Err(crate::Error::IanaIdNotFound(iana));
                }
                base.time_zone_id = mapped;
            }
            None => (),
        }

        match value.metazone {
            Some(MetazoneSpec::Id(id)) => base.metazone_id = Some(id),
            Some(MetazoneSpec::LocalDate(date)) => {
                let calc = MetazoneCalculator::new();
                base.maybe_calculate_metazone(&calc, &date.try_into()?);
            }
            None => (),
        }

        Ok(base)
    }
}

impl TryFrom<GmtOffsetSpec> for icu_timezone::GmtOffset {
    type Error = crate::Error;

    fn try_from(value: GmtOffsetSpec) -> Result<Self, Self::Error> {
        match value {
            GmtOffsetSpec::OffsetSeconds(sec) => Self::try_from_offset_seconds(sec),
            GmtOffsetSpec::Chars(chars) => Self::try_from_bytes(chars.as_bytes()),
        }
        .map_err(Self::Error::from)
    }
}

impl From<IsoFormatSpec> for icu_datetime::time_zone::IsoFormat {
    fn from(value: IsoFormatSpec) -> Self {
        match value {
            IsoFormatSpec::Basic => Self::Basic,
            IsoFormatSpec::Extended => Self::Extended,
            IsoFormatSpec::UtcBasic => Self::UtcBasic,
            IsoFormatSpec::UtcExtended => Self::UtcExtended,
        }
    }
}

impl From<IsoMinutesSpec> for icu_datetime::time_zone::IsoMinutes {
    fn from(value: IsoMinutesSpec) -> Self {
        match value {
            IsoMinutesSpec::Required => Self::Required,
            IsoMinutesSpec::Optional => Self::Optional,
        }
    }
}

impl From<IsoSecondsSpec> for icu_datetime::time_zone::IsoSeconds {
    fn from(value: IsoSecondsSpec) -> Self {
        match value {
            IsoSecondsSpec::Never => Self::Never,
            IsoSecondsSpec::Optional => Self::Optional,
        }
    }
}

impl From<FallbackSpec> for icu_datetime::time_zone::FallbackFormat {
    fn from(value: FallbackSpec) -> Self {
        match value {
            FallbackSpec::Iso8601 {
                format,
                minutes,
                seconds,
            } => Self::Iso8601(format.into(), minutes.into(), seconds.into()),
            FallbackSpec::LocalizedGmt => Self::LocalizedGmt,
        }
    }
}

impl From<FallbackSpec> for icu_datetime::time_zone::TimeZoneFormatterOptions {
    fn from(value: FallbackSpec) -> Self {
        icu_datetime::time_zone::FallbackFormat::from(value).into()
    }
}

impl IncludeSpec {
    pub fn apply(&self, f: &mut TimeZoneFormatter) -> Result<(), crate::Error> {
        match self {
            IncludeSpec::Iso8601 {
                format,
                minutes,
                seconds,
            } => f.include_iso_8601_format((*format).into(), (*minutes).into(), (*seconds).into()),
            IncludeSpec::GenericLocation => f.include_generic_location_format(),
            IncludeSpec::GenericNonLocationLong => f.include_generic_non_location_long(),
            IncludeSpec::GenericNonLocationShort => f.include_generic_non_location_short(),
            IncludeSpec::LocalizedGmt => f.include_localized_gmt_format(),
            IncludeSpec::SpecificNonLocationLong => f.include_specific_non_location_long(),
            IncludeSpec::SpecificNonLocationShort => f.include_specific_non_location_short(),
        }
        .map_err(crate::Error::from)
        .map(|_| ())
    }
}
