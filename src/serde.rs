use icu_datetime::{
    fieldsets::builder::{DateFields, FieldSetBuilder, ZoneStyle},
    options::{Alignment, SubsecondDigits, TimePrecision, YearStyle},
    Length,
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FieldSetBuilderSerde {
    pub length: Option<Length>,
    pub date_fields: Option<DateFields>,
    pub time_precision: Option<TimePrecisionSerde>,
    pub zone_style: Option<ZoneStyleSerde>,
    pub alignment: Option<Alignment>,
    pub year_style: Option<YearStyleSerde>,
}

impl From<FieldSetBuilderSerde> for FieldSetBuilder {
    fn from(value: FieldSetBuilderSerde) -> Self {
        let mut builder = Self::new();
        builder.length = value.length;
        builder.date_fields = value.date_fields;
        builder.time_precision = value.time_precision.map(Into::into);
        builder.zone_style = value.zone_style.map(Into::into);
        builder.alignment = value.alignment;
        builder.year_style = value.year_style.map(Into::into);
        builder
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum YearStyleSerde {
    Auto,
    Full,
    WithEra,
}

impl From<YearStyleSerde> for YearStyle {
    fn from(value: YearStyleSerde) -> Self {
        match value {
            YearStyleSerde::Auto => Self::Auto,
            YearStyleSerde::Full => Self::Full,
            YearStyleSerde::WithEra => Self::WithEra,
        }
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ZoneStyleSerde {
    SpecificLong,
    SpecificShort,
    LocalizedOffsetLong,
    LocalizedOffsetShort,
    GenericLong,
    GenericShort,
    Location,
    ExemplarCity,
}

impl From<ZoneStyleSerde> for ZoneStyle {
    fn from(value: ZoneStyleSerde) -> Self {
        match value {
            ZoneStyleSerde::SpecificLong => Self::SpecificLong,
            ZoneStyleSerde::SpecificShort => Self::SpecificShort,
            ZoneStyleSerde::LocalizedOffsetLong => Self::LocalizedOffsetLong,
            ZoneStyleSerde::LocalizedOffsetShort => Self::LocalizedOffsetShort,
            ZoneStyleSerde::GenericLong => Self::GenericLong,
            ZoneStyleSerde::GenericShort => Self::GenericShort,
            ZoneStyleSerde::Location => Self::Location,
            ZoneStyleSerde::ExemplarCity => Self::ExemplarCity,
        }
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimePrecisionSerde {
    Hour,
    Minute,
    Second,
    Subsecond1,
    Subsecond2,
    Subsecond3,
    Subsecond4,
    Subsecond5,
    Subsecond6,
    Subsecond7,
    Subsecond8,
    Subsecond9,
    MinuteOptional,
}

impl From<TimePrecisionSerde> for TimePrecision {
    fn from(value: TimePrecisionSerde) -> Self {
        match value {
            TimePrecisionSerde::Hour => TimePrecision::Hour,
            TimePrecisionSerde::Minute => TimePrecision::Minute,
            TimePrecisionSerde::Second => TimePrecision::Second,
            TimePrecisionSerde::Subsecond1 => TimePrecision::Subsecond(SubsecondDigits::S1),
            TimePrecisionSerde::Subsecond2 => TimePrecision::Subsecond(SubsecondDigits::S2),
            TimePrecisionSerde::Subsecond3 => TimePrecision::Subsecond(SubsecondDigits::S3),
            TimePrecisionSerde::Subsecond4 => TimePrecision::Subsecond(SubsecondDigits::S4),
            TimePrecisionSerde::Subsecond5 => TimePrecision::Subsecond(SubsecondDigits::S5),
            TimePrecisionSerde::Subsecond6 => TimePrecision::Subsecond(SubsecondDigits::S6),
            TimePrecisionSerde::Subsecond7 => TimePrecision::Subsecond(SubsecondDigits::S7),
            TimePrecisionSerde::Subsecond8 => TimePrecision::Subsecond(SubsecondDigits::S8),
            TimePrecisionSerde::Subsecond9 => TimePrecision::Subsecond(SubsecondDigits::S9),
            TimePrecisionSerde::MinuteOptional => TimePrecision::MinuteOptional,
        }
    }
}
