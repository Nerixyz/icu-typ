use std::str::FromStr;

use icu_calendar::Date;
use icu_datetime::DateFormatter;
use icu_locid::Locale;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Spec {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

#[derive(Deserialize)]
pub struct FormatOptions {
    pub locale: String,
    pub length: icu_datetime::options::length::Date,
}

pub fn format(
    date: Date<icu_calendar::Iso>,
    formatter_opts: FormatOptions,
) -> Result<Vec<u8>, crate::Error> {
    let locale = Locale::from_str(&formatter_opts.locale)?;
    let formatter = DateFormatter::try_new_with_length(&locale.into(), formatter_opts.length)?;

    Ok(crate::write::to_vec(formatter.format(&date.to_any())?))
}

impl TryFrom<Spec> for icu_calendar::Date<icu_calendar::Iso> {
    type Error = crate::Error;

    fn try_from(value: Spec) -> Result<Self, Self::Error> {
        Self::try_new_iso_date(value.year, value.month, value.day).map_err(Self::Error::from)
    }
}
