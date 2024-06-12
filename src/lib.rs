use wasm_minimal_protocol::*;

mod date;
mod datetime;
mod locale;
mod time;
mod timezone;
mod write;
mod zoned_datetime;

initiate_protocol!();

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CBOR deserialization error: {0}")]
    De(#[from] ciborium::de::Error<std::io::Error>),
    #[error("CBOR serialization error: {0}")]
    Ser(#[from] ciborium::ser::Error<std::io::Error>),
    #[error("ICU datetime error: {0}")]
    IcuDatetime(#[from] icu_datetime::Error),
    #[error("ICU calendar error: {0}")]
    IcuCalendar(#[from] icu_calendar::Error),
    #[error("ICU locale error: {0}")]
    IcuLocaleParse(#[from] icu_locid::Error),
    #[error("ICU timezone error: {0}")]
    IcuTimezone(#[from] icu_timezone::Error),
    #[error("The requested IANA timezone id wasn't found ({0})")]
    IanaIdNotFound(String),
    #[error("Formatting error: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("Failed to interpret as UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Failed to convert string {0} (source: '{1}')")]
    TinyStr(tinystr::TinyStrError, String),
}

macro_rules! make_formatter {
    ($name:ident from $module:ident) => {
        #[cfg_attr(target_arch = "wasm32", wasm_func)]
        pub fn $name(spec: &[u8], opts: &[u8]) -> Result<Vec<u8>, Error> {
            let spec = ciborium::from_reader::<$module::Spec, _>(spec)?;
            let opts = ciborium::from_reader::<$module::FormatOptions, _>(opts)?;

            $module::format(spec.try_into()?, opts)
        }
    };
}

make_formatter!(format_date from date);
make_formatter!(format_time from time);
make_formatter!(format_datetime from datetime);
make_formatter!(format_timezone from timezone);
make_formatter!(format_zoned_datetime from zoned_datetime);

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn locale_info(locale: &[u8]) -> Result<Vec<u8>, Error> {
    let locale = std::str::from_utf8(locale)?;

    locale::info(locale)
}
