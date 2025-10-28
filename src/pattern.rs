use std::str::FromStr;

use icu_calendar::{
    cal::{ChineseTraditional, KoreanTraditional},
    AnyCalendarKind, AsCalendar,
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
        HijriTabularEpoch, HijriTabularLeapYears, Indian, Japanese, JapaneseExtended, Persian, Roc,
    };
    match resolve_calendar_kind(&mut prefs) {
        CalendarKind::Buddhist => fmt_impl(spec, prefs, pattern, Buddhist),
        CalendarKind::Chinese => fmt_impl(spec, prefs, pattern, ChineseTraditional::new()),
        CalendarKind::Coptic => fmt_impl(spec, prefs, pattern, Coptic),
        CalendarKind::Dangi => fmt_impl(spec, prefs, pattern, KoreanTraditional::new()),
        CalendarKind::Ethiopian => fmt_impl(spec, prefs, pattern, Ethiopian::new()),
        CalendarKind::EthiopianAmeteAlem => fmt_impl(
            spec,
            prefs,
            pattern,
            Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem),
        ),
        CalendarKind::Gregorian => fmt_impl(spec, prefs, pattern, Gregorian),
        CalendarKind::Hebrew => fmt_impl(spec, prefs, pattern, Hebrew),
        CalendarKind::Indian => fmt_impl(spec, prefs, pattern, Indian),
        CalendarKind::HijriTabularTypeIIFriday => fmt_impl(
            spec,
            prefs,
            pattern,
            Hijri::<hijri::TabularAlgorithm>::new_tabular(
                HijriTabularLeapYears::TypeII,
                HijriTabularEpoch::Friday,
            ),
        ),
        CalendarKind::HijriSimulatedMecca => fmt_impl(
            spec,
            prefs,
            pattern,
            Hijri::<hijri::AstronomicalSimulation>::new_simulated_mecca(),
        ),
        CalendarKind::HijriTabularTypeIIThursday => fmt_impl(
            spec,
            prefs,
            pattern,
            Hijri::<hijri::TabularAlgorithm>::new_tabular(
                HijriTabularLeapYears::TypeII,
                HijriTabularEpoch::Thursday,
            ),
        ),
        CalendarKind::HijriUmmAlQura => fmt_impl(
            spec,
            prefs,
            pattern,
            Hijri::<hijri::UmmAlQura>::new_umm_al_qura(),
        ),
        CalendarKind::Japanese => fmt_impl(spec, prefs, pattern, Japanese::new()),
        CalendarKind::JapaneseExtended => fmt_impl(spec, prefs, pattern, JapaneseExtended::new()),
        CalendarKind::Persian => fmt_impl(spec, prefs, pattern, Persian),
        CalendarKind::Roc => fmt_impl(spec, prefs, pattern, Roc),
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

// reimplements `icu_datetime::scaffold::calendar::FormattableAnyCalendarKind::from_preferences`
// https://github.com/unicode-org/icu4x/blob/icu%402.1.0/components/datetime/src/scaffold/calendar.rs#L351-L367
fn resolve_calendar_kind(prefs: &mut DateTimeFormatterPreferences) -> CalendarKind {
    if let Some(kind) = CalendarKind::try_new(AnyCalendarKind::new((&*prefs).into())) {
        return kind;
    }

    // Calendar not supported by DateTimeFormatter
    // Currently this is CalendarAlgorithm::Iso8601, CalendarAlgorithm::Hijri(Rgsa)
    // Let AnyCalendarKind constructor select an appropriate fallback
    prefs.calendar_algorithm = None;
    if let Some(res) = CalendarKind::try_new(AnyCalendarKind::new((&*prefs).into())) {
        return res;
    }

    // unlike ICU, we default to the georgian calendar
    CalendarKind::Gregorian
}

// https://github.com/unicode-org/icu4x/blob/icu%402.1.0/components/datetime/src/scaffold/calendar.rs#L299-L319
enum CalendarKind {
    Buddhist,
    Chinese,
    Coptic,
    Dangi,
    Ethiopian,
    EthiopianAmeteAlem,
    Gregorian,
    Hebrew,
    Indian,
    HijriTabularTypeIIFriday,
    HijriSimulatedMecca,
    HijriTabularTypeIIThursday,
    HijriUmmAlQura,
    Japanese,
    JapaneseExtended,
    Persian,
    Roc,
}

impl CalendarKind {
    fn try_new(kind: AnyCalendarKind) -> Option<CalendarKind> {
        match kind {
            AnyCalendarKind::Buddhist => Some(Self::Buddhist),
            AnyCalendarKind::Chinese => Some(Self::Chinese),
            AnyCalendarKind::Coptic => Some(Self::Coptic),
            AnyCalendarKind::Dangi => Some(Self::Dangi),
            AnyCalendarKind::Ethiopian => Some(Self::Ethiopian),
            AnyCalendarKind::EthiopianAmeteAlem => Some(Self::EthiopianAmeteAlem),
            AnyCalendarKind::Gregorian => Some(Self::Gregorian),
            AnyCalendarKind::Hebrew => Some(Self::Hebrew),
            AnyCalendarKind::Indian => Some(Self::Indian),
            AnyCalendarKind::HijriTabularTypeIIFriday => Some(Self::HijriTabularTypeIIFriday),
            AnyCalendarKind::HijriSimulatedMecca => Some(Self::HijriSimulatedMecca),
            AnyCalendarKind::HijriTabularTypeIIThursday => Some(Self::HijriTabularTypeIIThursday),
            AnyCalendarKind::HijriUmmAlQura => Some(Self::HijriUmmAlQura),
            AnyCalendarKind::Japanese => Some(Self::Japanese),
            AnyCalendarKind::JapaneseExtended => Some(Self::JapaneseExtended),
            AnyCalendarKind::Persian => Some(Self::Persian),
            AnyCalendarKind::Roc => Some(Self::Roc),

            AnyCalendarKind::Iso => None,
            _ => None,
        }
    }
}
