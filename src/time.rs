use std::str::FromStr;

use icu_calendar::Time;
use icu_datetime::TimeFormatter;
use icu_locid::Locale;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Spec {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    #[serde(default)]
    pub nanosecond: u32,
}

#[derive(Deserialize)]
pub struct FormatOptions {
    pub locale: String,
    pub length: icu_datetime::options::length::Time,
}

pub fn format(time: Time, formatter_opts: FormatOptions) -> Result<Vec<u8>, crate::Error> {
    let locale = Locale::from_str(&formatter_opts.locale)?;
    let formatter = TimeFormatter::try_new_with_length(&locale.into(), formatter_opts.length)?;

    Ok(crate::write::to_vec(formatter.format(&time)))
}

impl TryFrom<Spec> for icu_calendar::Time {
    type Error = crate::Error;

    fn try_from(value: Spec) -> Result<Self, Self::Error> {
        Ok(Self {
            hour: value.hour.try_into()?,
            minute: value.minute.try_into()?,
            second: value.second.try_into()?,
            nanosecond: value.nanosecond.try_into()?,
        })
    }
}
