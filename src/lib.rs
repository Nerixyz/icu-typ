use wasm_minimal_protocol::*;

// mod date;
mod datetime;
// mod locale;
// mod time;
// mod timezone;
mod write;
// mod zoned_datetime;

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
    #[error("Failed to create formatting fields: {0}")]
    CompositeError(icu_datetime::fieldsets::serde::CompositeFieldSetSerdeError),
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
    #[error("The provided time zone (ID) will never have the provided offset")]
    OffsetMismatch,
    #[error("Both IANA and Bcp47 IDs were specifies, expected at most one")]
    IanaAndBcp47,

    #[error("Some values are missing to format the date with the specified fields")]
    MissingValues,
}

macro_rules! make_formatter {
    ($name:ident from $module:ident) => {
        #[cfg_attr(target_arch = "wasm32", wasm_func)]
        pub fn $name(spec: &[u8], opts: &[u8]) -> Result<Vec<u8>, Error> {
            let spec = ciborium::from_reader::<$module::Spec, _>(spec)
                .map_err(|it| Error::De("spec", it))?;
            let opts = ciborium::from_reader::<$module::FormatOptions, _>(opts)
                .map_err(|it| Error::De("options", it))?;

            $module::format(spec, opts)
        }
    };
}

// make_formatter!(format_date from date);
// make_formatter!(format_time from time);
make_formatter!(format_datetime from datetime);
// make_formatter!(format_timezone from timezone);
// make_formatter!(format_zoned_datetime from zoned_datetime);

// #[cfg_attr(target_arch = "wasm32", wasm_func)]
// pub fn locale_info(locale: &[u8]) -> Result<Vec<u8>, Error> {
//     let locale = std::str::from_utf8(locale)?;

//     locale::info(locale)
// }
