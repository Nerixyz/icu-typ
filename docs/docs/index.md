# icu-datetime

This library is a wrapper around [ICU4X](https://github.com/unicode-org/icu4x)' `datetime` formatting for Typst which provides internationalized formatting for dates, times, and timezones.

## Usage

```typst +preview(vertical mode="basic")
#import "@preview/icu-datetime:0.1.1": fmt-date, fmt-time, fmt-datetime

// These functions may change at any time
#import "@preview/icu-datetime:0.1.1": experimental
#import experimental: fmt-timezone, fmt-zoned-datetime

This documentation was built #fmt-date(datetime.today(), length: "full").
```

## Date

```typst +preview
#let day = datetime(
  year: 2024,
  month: 5,
  day: 31,
)

#fmt-date(day, locale: "km", length: "full") \
#fmt-date(day, locale: "af", length: "full") \
#fmt-date(day, locale: "za", length: "full") \
```

## Time

```typst +preview
#let time = datetime(
  hour: 18,
  minute: 2,
  second: 23,
)

#fmt-time(time, locale: "id", length: "medium") \
#fmt-time(time, locale: "en", length: "medium") \
#fmt-time(time, locale: "ga", length: "medium") \
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

#fmt-datetime(dt, locale: "ru", date-length: "full") \
#fmt-datetime(dt, locale: "en-US", date-length: "full") \
#fmt-datetime(dt, locale: "zh-Hans-CN", date-length: "full") \
#fmt-datetime(dt, locale: "ar", date-length: "full") \
#fmt-datetime(dt, locale: "fi", date-length: "full")
```

## Timezones (🚧 experimental)

```typst +preview
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
  zone-variant: "st", // standard
)

#fmt-timezone(
  ..tz,
  local-date: dt,
  includes: "specific-non-location-long"
) \
#fmt-timezone(
  ..tz,
  local-date: dt,
  includes: (
    iso8601: (
      format: "utc-extended",
      minutes: "required",
      seconds: "optional",
    )
  )
)
```

## Zoned Datetimes (🚧 experimental)

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
  zone-variant: "st", // standard
)

#fmt-zoned-datetime(dt, tz) \
#fmt-zoned-datetime(dt, tz, locale: "lv") \
#fmt-zoned-datetime(
    dt,
    tz,
    locale: "en-CA-u-hc-h24-ca-buddhist"
)
```
