use std::str::FromStr;

use icu_calendar::DateTime;
use icu_datetime::ZonedDateTimeFormatter;
use icu_locid::Locale;
use serde::Deserialize;

use crate::datetime::Baggable;

#[derive(Deserialize)]
pub struct Spec {
    pub datetime: crate::datetime::Spec,
    pub timezone: crate::timezone::Spec,
}

#[derive(Deserialize)]
pub struct FormatOptions {
    pub locale: String,
    pub date: Option<icu_datetime::options::length::Date>,
    pub time: Option<icu_datetime::options::length::Time>,
    pub fallback: crate::timezone::FallbackSpec,
}

pub fn format(
    (datetime, timezone): (DateTime<icu_calendar::Iso>, icu_timezone::CustomTimeZone),
    formatter_opts: FormatOptions,
) -> Result<Vec<u8>, crate::Error> {
    let locale = Locale::from_str(&formatter_opts.locale)?;
    let formatter = ZonedDateTimeFormatter::try_new(
        &locale.into(),
        formatter_opts.bag().into(),
        formatter_opts.fallback.into(),
    )?;

    Ok(crate::write::to_vec(
        formatter.format(&datetime.to_any(), &timezone)?,
    ))
}

impl TryFrom<Spec> for (DateTime<icu_calendar::Iso>, icu_timezone::CustomTimeZone) {
    type Error = crate::Error;

    fn try_from(value: Spec) -> Result<Self, Self::Error> {
        let (dt, mut tz): (_, icu_timezone::CustomTimeZone) =
            (value.datetime.try_into()?, value.timezone.try_into()?);
        let mzc = icu_timezone::MetazoneCalculator::new();
        tz.maybe_calculate_metazone(&mzc, &dt);

        Ok((dt, tz))
    }
}

impl Baggable for FormatOptions {
    fn bag(&self) -> icu_datetime::options::length::Bag {
        (self.date, self.time).bag()
    }
}
