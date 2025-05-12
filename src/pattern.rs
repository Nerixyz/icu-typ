use std::str::FromStr;

use icu_calendar::Gregorian;
use icu_datetime::{
    fieldsets::enums::CompositeFieldSet,
    pattern::{DateTimePattern, FixedCalendarDateTimeNames},
};
use icu_locale_core::Locale;
use icu_time::ZonedDateTime;

use crate::format::{Spec, SpecifiedZonedDateTime};

pub fn format(pattern_str: &str, locale: &str, spec: Spec) -> Result<Vec<u8>, crate::Error> {
    let spec: SpecifiedZonedDateTime = spec.try_into()?;
    let converted = ZonedDateTime {
        date: spec.value.date.to_calendar(Gregorian),
        time: spec.value.time,
        zone: spec.value.zone,
    };

    let prefs = Locale::from_str(locale)?.into();
    let pattern = DateTimePattern::try_from_pattern_str(pattern_str)?;
    let mut names = FixedCalendarDateTimeNames::<Gregorian, CompositeFieldSet>::try_new(prefs)?;

    crate::write::try_to_vec(&names.include_for_pattern(&pattern)?.format(&converted))
        .map_err(crate::Error::FormattedPatternError)
}
