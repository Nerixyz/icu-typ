use icu_time::zone::{UtcOffset, UtcOffsets};
use wasm_minimal_protocol::*;

mod format;
mod locale;
mod serde;
mod write;

initiate_protocol!();

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CBOR deserialization error for {0}: {1}")]
    De(&'static str, ciborium::de::Error<std::io::Error>),
    #[error("CBOR serialization error: {0}")]
    Ser(#[from] ciborium::ser::Error<std::io::Error>),
    #[error("ICU locale error: {0}")]
    IcuLocaleParse(#[from] icu_locale_core::ParseError),
    #[error("Formatting error: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("Failed to interpret as UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Failed to convert string {0} (source: '{1}')")]
    TinyStr(tinystr::ParseError, String),

    #[error("Failed to load date-time formatter: {0}")]
    LoadDateTimeFormatter(icu_datetime::DateTimeFormatterLoadError),
    #[error("Failed to create formatter: {0}")]
    CompositeError(#[from] icu_datetime::fieldsets::builder::BuilderError),
    #[error("Invalid field range: {0}")]
    DateRange(icu_calendar::RangeError),

    #[error("A partial date was provided - either year, month, and day must be provided or none")]
    PartialDate,
    #[error("A partial time was provided - when specifying time, hour must always be present")]
    PartialTime,

    #[error("The specified time zone was not found")]
    UnknownIana,
    #[error("The time zone offset was invalid. Must be within ±18:00:00.")]
    InvalidOffset,
    #[error("The provided time zone (ID) will never have the provided offset - it has these offsets: {0}")]
    OffsetMismatch(InvalidUtcOffsets),
    #[error("Both IANA and Bcp47 IDs were specifies, expected at most one")]
    IanaAndBcp47,

    #[error("Some values are missing to format the date with the specified fields")]
    MissingValues,
}

#[derive(Debug)]
pub struct InvalidUtcOffsets(pub UtcOffsets);

impl std::fmt::Display for InvalidUtcOffsets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_offset(o: UtcOffset, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if o.is_non_negative() {
                write!(f, "+")?;
            }
            write!(f, "{:02}", o.hours_part())?;
            let has_secs = o.seconds_part() != 0;
            let has_mins = o.seconds_part() != 0;
            if has_mins || has_secs {
                write!(f, ":{:02}", o.minutes_part())?;
                if has_secs {
                    write!(f, ":{:02}", o.seconds_part())?;
                }
            }
            Ok(())
        }

        write!(f, "standard=")?;
        write_offset(self.0.standard, f)?;
        write!(f, ", daylight=")?;
        match self.0.daylight {
            Some(o) => write_offset(o, f),
            None => write!(f, "n/a"),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn format(spec: &[u8], locale: &[u8], opts: &[u8]) -> Result<Vec<u8>, Error> {
    let spec =
        ciborium::from_reader::<format::Spec, _>(spec).map_err(|it| Error::De("spec", it))?;
    let locale = std::str::from_utf8(locale)?;
    let builder = ciborium::from_reader::<serde::FieldSetBuilderSerde, _>(opts)
        .map_err(|it| Error::De("opts", it))?;

    format::format(spec, &locale, builder.into())
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn locale_info(locale: &[u8]) -> Result<Vec<u8>, Error> {
    let locale = std::str::from_utf8(locale)?;

    locale::info(locale)
}
