# icu-datetime

This library is a wrapper around [ICU4X](https://github.com/unicode-org/icu4x)' `datetime` formatting for Typst which provides internationalized formatting for dates, times, and timezones.

## Usage

```typst +preview(vertical fake)
#import "@preview/icu-datetime:0.2.0": icu

This documentation was built on #icu.fmt(datetime.today()).
```

For detailed documentation, see [`fmt`- Format Date and Time](fmt.md).

## Date

```typst +preview
#let day = datetime(
  year: 2024,
  month: 5,
  day: 31,
)

#icu.fmt(day, locale: "km", date-fields: "YMD") \
#icu.fmt(day, locale: "af", date-fields: "YMD") \
#icu.fmt(day, locale: "za", date-fields: "YMD") \
```

## Time

```typst +preview
#let time = datetime(
  hour: 18,
  minute: 2,
  second: 23,
)

#icu.fmt(time, locale: "id", time-precision: "second") \
#icu.fmt(time, locale: "en", time-precision: "second") \
#icu.fmt(time, locale: "ga", time-precision: "second") \
```

## Date and Time

```typst +preview(vertical)
#let dt = datetime(
  year: 2024,
  month: 5,
  day: 31,
  hour: 18,
  minute: 2,
  second: 23,
)

#icu.fmt(dt, locale: "ru", length: "long") \
#icu.fmt(dt, locale: "en-US", length: "long") \
#icu.fmt(dt, locale: "zh-Hans-CN", length: "long") \
#icu.fmt(dt, locale: "ar", length: "long") \
#icu.fmt(dt, locale: "fi", length: "long")
```

## Timezones

```typst +preview
#let tz = (
  offset: "-07",
  iana: "America/Los_Angeles",
)

#icu.fmt(
  datetime.today(), zone: tz,
  zone-style: "specific-long"
) \
#icu.fmt(
  datetime.today(), zone: tz,
  zone-style: "generic-short"
)
```

## Zoned Datetimes

```typst +preview(vertical)
#let dt = datetime(
  year: 2024,
  month: 5,
  day: 31,
  hour: 18,
  minute: 2,
  second: 23,
)
#let tz = (
  offset: "-07",
  iana: "America/Los_Angeles",
)
#let opts = (
  zone: tz,
  date-fields: "YMDE",
  time-precision: "second",
  length: "long"
)

#icu.fmt(dt, ..opts, zone-style: "generic-short") \
#icu.fmt(dt, ..opts,
  zone-style: "localized-offset-short",
  locale: "lv"
) \
#icu.fmt(dt, ..opts,
  zone-style: "exemplar-city",
  locale: "en-CA-u-hc-h24-ca-buddhist"
)
```
