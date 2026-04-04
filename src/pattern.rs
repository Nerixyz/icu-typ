use std::str::FromStr;

use icu_calendar::{
    cal::{ChineseTraditional, KoreanTraditional},
    preferences::{CalendarAlgorithm, CalendarPreferences, HijriCalendarAlgorithm},
    AsCalendar,
};
use icu_datetime::{
    fieldsets::enums::CompositeFieldSet,
    pattern::{DateTimePattern, FixedCalendarDateTimeNames},
    scaffold::CldrCalendar,
    DateTimeFormatterPreferences,
};
use icu_locale_core::Locale;
use icu_provider::DataProvider;
use icu_time::ZonedDateTime;

use crate::format::{Spec, SpecifiedZonedDateTime};

pub fn format(pattern_str: &str, locale: &str, spec: Spec) -> Result<Vec<u8>, crate::Error> {
    let spec: SpecifiedZonedDateTime = spec.try_into()?;

    let prefs = Locale::from_str(locale)?.into();
    let pattern = DateTimePattern::try_from_pattern_str(pattern_str)?;

    // waiting on https://github.com/unicode-org/icu4x/issues/6107
    format_with_calendar(&spec, prefs, &pattern)
}

fn format_with_calendar(
    spec: &SpecifiedZonedDateTime,
    mut prefs: DateTimeFormatterPreferences,
    pattern: &DateTimePattern,
) -> Result<Vec<u8>, crate::Error> {
    use icu_calendar::cal::{
        hijri, Buddhist, Coptic, Ethiopian, EthiopianEraStyle, Gregorian, Hebrew, Hijri,
        HijriTabularEpoch, HijriTabularLeapYears, Indian, Japanese, Persian, Roc,
    };
    // https://github.com/unicode-org/icu4x/blob/icu%402.2.0/components/datetime/src/scaffold/calendar.rs#L449-L488
    match CalendarPreferences::from(&prefs).resolved_algorithm() {
        CalendarAlgorithm::Buddhist => fmt_impl(spec, prefs, pattern, Buddhist),
        CalendarAlgorithm::Chinese => fmt_impl(spec, prefs, pattern, ChineseTraditional::new()),
        CalendarAlgorithm::Coptic => fmt_impl(spec, prefs, pattern, Coptic),
        CalendarAlgorithm::Dangi => fmt_impl(spec, prefs, pattern, KoreanTraditional::new()),
        CalendarAlgorithm::Ethiopic => fmt_impl(spec, prefs, pattern, Ethiopian::new()),
        CalendarAlgorithm::Ethioaa => fmt_impl(
            spec,
            prefs,
            pattern,
            Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem),
        ),
        CalendarAlgorithm::Gregory => fmt_impl(spec, prefs, pattern, Gregorian),
        CalendarAlgorithm::Hebrew => fmt_impl(spec, prefs, pattern, Hebrew),
        CalendarAlgorithm::Indian => fmt_impl(spec, prefs, pattern, Indian),
        CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Civil)) => fmt_impl(
            spec,
            prefs,
            pattern,
            Hijri::<hijri::TabularAlgorithm>::new_tabular(
                HijriTabularLeapYears::TypeII,
                HijriTabularEpoch::Friday,
            ),
        ),
        CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Tbla)) => fmt_impl(
            spec,
            prefs,
            pattern,
            Hijri::<hijri::TabularAlgorithm>::new_tabular(
                HijriTabularLeapYears::TypeII,
                HijriTabularEpoch::Thursday,
            ),
        ),
        CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Umalqura)) => fmt_impl(
            spec,
            prefs,
            pattern,
            Hijri::<hijri::UmmAlQura>::new_umm_al_qura(),
        ),
        CalendarAlgorithm::Japanese => fmt_impl(spec, prefs, pattern, Japanese::new()),
        CalendarAlgorithm::Persian => fmt_impl(spec, prefs, pattern, Persian),
        CalendarAlgorithm::Roc => fmt_impl(spec, prefs, pattern, Roc),
        CalendarAlgorithm::Iso8601 | CalendarAlgorithm::Hijri(_) => {
            // unsupported
            prefs.calendar_algorithm = None;
            format_with_calendar(spec, prefs, pattern)
        }
        // unknown
        _ => fmt_impl(spec, prefs, pattern, Gregorian),
    }
}

fn fmt_impl<C>(
    spec: &SpecifiedZonedDateTime,
    prefs: DateTimeFormatterPreferences,
    pattern: &DateTimePattern,
    cal: C,
) -> Result<Vec<u8>, crate::Error>
where
    C: AsCalendar<Calendar = C> + CldrCalendar + icu_calendar::Calendar,
    icu_datetime::provider::Baked: DataProvider<<C as CldrCalendar>::YearNamesV1>,
    icu_datetime::provider::Baked: DataProvider<<C as CldrCalendar>::MonthNamesV1>,
{
    let mut names = FixedCalendarDateTimeNames::<C, CompositeFieldSet>::try_new(prefs)?;
    let converted = ZonedDateTime {
        date: spec.value.date.to_calendar(cal),
        time: spec.value.time,
        zone: spec.value.zone,
    };

    crate::write::try_to_vec(&names.include_for_pattern(pattern)?.format(&converted))
        .map_err(crate::Error::FormattedPatternError)
}
