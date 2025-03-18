# `fmt` - Format Date and Time

```typst-code
import "@preview/icu-datetime:0.2.0" as icu

let fmt(
  dt,
  zone: none,

  locale: "en",

  length: none,
  date-fields: none,
  time-precision: none,
  zone-style: none,
  alignment: none,
  year-style: none,
)
```

Formats a date and time in some [`locale`](#locale). Dates are assumed to be ISO dates.

## Arguments

### `dt`

The date and time to format. This can be a [`datetime`][datetime] or a dictionary with `year`, `month`, `day`, `hour`, `minute`, `second`, and (optionally) `nanosecond`.

example{

```typst +preview
#icu.fmt(datetime(
  year: 2024,
  month: 5,
  day: 31,
  hour: 18,
  minute: 2,
  second: 23,
)) \
#icu.fmt(( // (1)!
  year: 1974,
  month: 8,
  day: 20,
  hour: 10,
  minute: 40,
  second: 38,
), locale: "mt")
```

1. Datetime passed as a dictionary

}example

### `zone`

A time zone passed as a dictionary with `offset` (required), `bcp47` or `iana`.

- `offset`: The UTC offset either specified as a string (e.g. `#!typst-code "+05"`) or as an integer specifying the seconds (`#!typst-code 18000`).
- `bcp47`: BCP-47 timezone ID (e.g. `#!typst-code "iodga"` (IANA: Indian/Chagos) - see [timezone.xml](https://github.com/unicode-org/cldr/blob/main/common/bcp47/timezone.xml)). This is mutually exclusive with `iana`.
- `iana`: IANA TZ identifier (e.g. `#!typst-code "Brazil/West"` - see [IANA](https://www.iana.org/time-zones) and [Wikipedia](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)). This is mutually exclusive with `bcp47`.

example{

```typst +preview
#let f(tz) = icu.fmt(
  datetime.today(), // (1)!
  zone: tz,
  zone-style: "specific-long"
)

- #f((offset: "+04"))
- #f((offset: "+04", iana: "Asia/Baku"))
- #f((offset: "+04", bcp47: "azbak"))
- #f((offset: "+04", bcp47: "aedxb"))
- #f((offset: "-09:30"))
- #f((offset: "-09:30", iana: "Pacific/Marquesas"))
```

1. Date specified to resolve metazones.

}example

### `locale`

The locale to use when formatting the datetime. A [Unicode Locale Identifier]. As with [dates](./fmt-date.md#locale), `ca` can be set to a [bcp47 calendar name](https://github.com/unicode-org/cldr/blob/main/common/bcp47/calendar.xml).

example{

```typst +preview
#let dt = datetime(
  year: 2024,
  month: 5,
  day: 31,
  hour: 18,
  minute: 2,
  second: 23,
)

- #icu.fmt(dt, locale: "en")
- #icu.fmt(dt, locale: "ko")
- #icu.fmt(dt, locale: "ar-u-ca-islamic")
- #icu.fmt(dt, locale: "mk")
- #icu.fmt(dt, locale: "so")
- #icu.fmt(dt, locale: "fo")
- #icu.fmt(dt, locale: "vi")
- #icu.fmt(dt, locale: "vi-u-ca-buddhist")
```

}example

### `length`

The length of the formatted string (`#!typst-code "long"`, `#!typst-code "medium"` (default), or `#!typst-code "short"`).

example{

```typst +preview
#let dt = datetime(
  year: 2024,
  month: 5,
  day: 31,
  hour: 18,
  minute: 2,
  second: 23,
)

*Long*
- #icu.fmt(dt, length: "long")
- #icu.fmt(dt, length: "long", locale: "hi")
- #icu.fmt(dt, length: "long", locale: "da")

*Medium*
- #icu.fmt(dt, length: "medium")
- #icu.fmt(dt, length: "medium", locale: "ja")
- #icu.fmt(dt, length: "medium", locale: "pt")

*Short*
- #icu.fmt(dt, length: "short")
- #icu.fmt(dt, length: "short", locale: "bn")
- #icu.fmt(dt, length: "short", locale: "az")
```

}example

### `date-fields`

The fields of the date to include in the formatted string. `#!typst-code "D"` (day of month), `#!typst-code "MD"`, `#!typst-code "YMD"`, `#!typst-code "DE"`, `#!typst-code "MDE"`, `#!typst-code "YMDE"`, `#!typst-code "E"` (weekday), `#!typst-code "M" (month)`, `#!typst-code "YM"`, `#!typst-code "Y"` (year), or `#!typst-code none`.

Defaults to `#!typst-code "YMD"` if neither `time-precison` nor `zone-style` are specified - otherwise this defaults to `none` and the date isn't included in the output.

The avialable options are also provided in `fields` as a dictionary.

example{

```typst +preview
#table(
  columns: 2,
  [`date-fields`], [Output],
  ..icu
    .fields // (1)!
    .values()
    .map(v => (
      v,
      icu.fmt(
        (year: 2024, month: 5, day: 31),
        date-fields: v,
        length: "long",
      ),
    ))
    .flatten()
)
```

1. `fields` is a dictionary that contains all possible values for `date-fiels`.

}example

### `time-precision`

How precise to display the time. "hour", "minute", "second", "subsecond{n}" (n subsecond digits), "minute-optional" ("hour" if `minutes == 0`, otherwise "minute"), or `none`. Defaults to "minute" if neither `date-fields` nor `zone-style` are specified - otherwise this defaults to `none` and the time isn't included in the output. The avialable options are also provided in `time-precision` as a dictionary.

example{

```typst +preview(vertical)
#table(
  columns: 3,
  [`time-precision`], [American English], [British English],
  ..icu
    .time-precision // (1)!
    .values()
    .map(style => (
      style,
      ("en-US", "en-GB")
        .map(locale => icu.fmt(
          (hour: 16, minute: 0, second: 3, nanosecond: 123456789),
          time-precision: style,
          locale: locale,
        ))
        .flatten(),
    ))
    .flatten()
)
```

1. `time-precision` is a dictionary that contains all possible values for `time-precision`.

}example

### `zone-style`

How to format the timezone (if any). "specific-long", "specific-short", "localized-offset-long", "localized-offset-short", "generic-long", "generic-short", "location", "exemplar-city", or `none`. Defaults to `none`. The avialable options are also provided in `zone-style` as a dictionary.

example{

```typst +preview(vertical)
#table(
  columns: 4,
  [`zone-style`], [English], [Indonesian], [Japanese],
  ..icu
    .zone-styles // (1)!
    .values()
    .map(style => (
      style,
      ("en", "id", "jp")
        .map(locale => icu.fmt(
          datetime.today(),
          zone: (offset: "+08", iana: "Asia/Makassar"),
          zone-style: style,
          locale: locale,
        ))
        .flatten(),
    ))
    .flatten()
)
```

1. `zone-styles` is a dictionary that contains all possible values for `zone-style`.

}example

### `alignment`

How to align (pad) the formatted string. "auto", "column", or `none` (default, implies "auto").

example{

```typst +preview
#let f(date) = icu.fmt(
  date,
  date-fields: "YMD",
  alignment: "column",
  length: "short",
)

- #f((year: 2024, month: 5, day: 2))
- #f((year: 2024, month: 6, day: 16))
- #f((year: 2024, month: 8, day: 23))
- #f((year: 2006, month: 12, day: 2))

```

}example

### `year-style`

How to format the year and the era. "auto", "full", "with-era", `none` (default, implies "auto").

example{

```typst +preview(vertical)
#table(
  columns: 5,
  [`year-style`], [2024], [-128], [1984], [1847],
  ..icu
    .year-style // (1)!
    .values()
    .map(style => (
      style,
      ..(2024, -128, 1984, 1847).map(y => icu.fmt(
        (year: y, month: 1, day: 1),
        date-fields: "Y",
        year-style: style,
        length: "short",
      )),
    ))
    .flatten()
)
```

1. `year-style` is a dictionary that contains all possible values for `year-style`.

}example

[datetime]: https://typst.app/docs/reference/foundations/datetime/
[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
