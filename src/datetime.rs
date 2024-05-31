use std::str::FromStr;

use icu_calendar::DateTime;
use icu_datetime::DateTimeFormatter;
use icu_locid::Locale;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Spec {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    #[serde(default)]
    pub nanosecond: u32,
}

#[derive(Deserialize)]
pub struct FormatOptions {
    pub locale: String,
    pub date: Option<icu_datetime::options::length::Date>,
    pub time: Option<icu_datetime::options::length::Time>,
}

pub fn format(
    datetime: DateTime<icu_calendar::Iso>,
    formatter_opts: FormatOptions,
) -> Result<Vec<u8>, crate::Error> {
    let locale = Locale::from_str(&formatter_opts.locale)?;
    let formatter = DateTimeFormatter::try_new(&locale.into(), formatter_opts.bag().into())?;

    Ok(crate::write::to_vec(formatter.format(&datetime.to_any())?))
}

impl TryFrom<Spec> for icu_calendar::DateTime<icu_calendar::Iso> {
    type Error = crate::Error;

    fn try_from(value: Spec) -> Result<Self, Self::Error> {
        Ok(Self {
            date: icu_calendar::Date::try_new_iso_date(value.year, value.month, value.day)?,
            time: icu_calendar::Time {
                hour: value.hour.try_into()?,
                minute: value.minute.try_into()?,
                second: value.second.try_into()?,
                nanosecond: value.nanosecond.try_into()?,
            },
        })
    }
}

pub trait Baggable {
    fn bag(&self) -> icu_datetime::options::length::Bag;
}

impl Baggable
    for (
        Option<icu_datetime::options::length::Date>,
        Option<icu_datetime::options::length::Time>,
    )
{
    fn bag(&self) -> icu_datetime::options::length::Bag {
        use icu_datetime::options::length::Bag;
        match (self.0, self.1) {
            (Some(date), Some(time)) => Bag::from_date_time_style(date, time),
            (Some(date), None) => Bag::from_date_style(date),
            (None, Some(time)) => Bag::from_time_style(time),
            (None, None) => Bag::empty(),
        }
    }
}

impl Baggable for FormatOptions {
    fn bag(&self) -> icu_datetime::options::length::Bag {
        (self.date, self.time).bag()
    }
}
