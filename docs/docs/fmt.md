# `fmt` - Format Date and Time

```typst-code
import "@preview/icu-datetime:0.2.0" as icu

let fmt(
  dt,
  zone: none,

  locale: "en",

  length: none,
  date-fields: auto,
  time-precision: auto,
  zone-style: auto,
  alignment: none,
  year-style: none,
)
```

Formats a date and time in some [`locale`](#locale). Dates are assumed to be ISO dates.

## Defaults

The function tries to infer the intended format automatically if [`date-fields`](#date-fields), [`time-precision`](#time-precision), and [`zone-style`](#zone-style) _all_ use their default values (`auto`):

-   If [`dt`](#dt) has date fields (`year`, `month`, `day`), then [`date-fields`](#date-fields) will be set to `#!typst-code "YMD"`
-   If [`dt`](#dt) has time fields (`hour`, `minute`, `second`), then [`time-precision`](#time-precision) will be set to `#!typst-code "minute"`
-   If [`zone`](#zone) has a value, then [`zone-style`](#zone-style) will be set to `#!typst-code "localized-offset-short"`

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

A zone passed as a dictionary with `offset` (required), `bcp47` or `iana`.

-   `offset`: The UTC offset either specified as a string (e.g. `#!typst-code "+05"`) or as an integer specifying the seconds (`#!typst-code 18000`).
-   `bcp47`: BCP-47 timezone ID (e.g. `#!typst-code "iodga"` (IANA: Indian/Chagos) - see [timezone.xml](https://github.com/unicode-org/cldr/blob/main/common/bcp47/timezone.xml)). This is mutually exclusive with `iana`.
-   `iana`: IANA TZ identifier (e.g. `#!typst-code "Brazil/West"` - see [IANA](https://www.iana.org/time-zones) and [Wikipedia](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)). This is mutually exclusive with `bcp47`.

If zones are formatted on their own, `dt` _can_ be an empty dictionary (`#!typst-code (:)`). However, when specified, the date and time will still be used to resolve the zone variant (standard/daylight). Otherwise, the standard variant will be used. Note that this doesn't resolve the variant at the specified date but the variant at the specified `offset`.

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

1. Date specified to resolve zone variants.

}example

### `locale`

The locale to use when formatting the datetime. A [Unicode Locale Identifier]. Notably, this can be used to set the calendar by setting `ca` to a [bcp47 calendar name](https://github.com/unicode-org/cldr/blob/main/common/bcp47/calendar.xml).

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
- #icu.fmt(dt, locale: "en-TH")
- #icu.fmt(dt, locale: "mk")
- #icu.fmt(dt, locale: "so")
- #icu.fmt(dt, locale: "fo")
- #icu.fmt(dt, locale: "vi")
- #icu.fmt(dt, locale: "vi-u-ca-buddhist")
```

}example

### `length`

The length of the formatted string (`#!typst-code "long"`, `#!typst-code "medium"` (default), or `#!typst-code "short"`).

The avialable options are also provided in [`length`](./constants.md#length) as a dictionary.

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

he fields of the date to include in the formatted string. `#!typst-code "D"` (day of month), `#!typst-code "MD"`, `#!typst-code "YMD"`, `#!typst-code "DE"`, `#!typst-code "MDE"`, `#!typst-code "YMDE"`, `#!typst-code "E"` (weekday), `#!typst-code "M"` (month), `#!typst-code "YM"`, `#!typst-code "Y"` (year), `#!typst-code none`, or `#!typst-code auto` (default, see [defaults](#defaults)).

The avialable options are also provided in [`date-fields`](./constants.md#date-fields) as a dictionary.

example{

```typst +preview
#table(
  columns: 2,
  [`date-fields`], [Output],
  ..icu
    .date-fields // (1)!
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

1. [`date-fields`](./constants.md#date-fields) is a dictionary that contains all possible values for `date-fiels`.

}example

### `time-precision`

How precise to display the time. `#!typst-code "hour"`, `#!typst-code "minute"`, `#!typst-code "second"`, `#!typst-code "subsecond{n}"` (n subsecond digits), `#!typst-code "minute-optional"` (`#!typst-code "hour"` if `#!typst-code minutes == 0`, otherwise `#!typst-code "minute"`), `none`, or `auto` (default, see [defaults](#defaults)).

The avialable options are also provided in [`time-precision`](./constants.md#time-precision) as a dictionary.

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

1. [`time-precision`](./constants.md#time-precision) is a dictionary that contains all possible values for `time-precision`.

}example

### `zone-style`

How to format the timezone (if any). `#!typst-code "specific-long"`, `#!typst-code "specific-short"`, `#!typst-code "localized-offset-long"`, `#!typst-code "localized-offset-short"`, `#!typst-code "generic-long"`, `#!typst-code "generic-short"`, `#!typst-code "location"`, `#!typst-code "exemplar-city"`, `none`, or `auto` (default, see [defaults](#defaults)).

The avialable options are also provided in [`zone-styles`](./constants.md#zone-styles) as a dictionary.

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

1. [`zone-styles`](./constants.md#zone-styles) is a dictionary that contains all possible values for `zone-style`.

}example

### `alignment`

How to align (pad) the formatted string. `#!typst-code "auto"`, `#!typst-code "column"`, or `none` (default, implies `#!typst-code "auto"`).

The avialable options are also provided in [`alignment`](./constants.md#alignment) as a dictionary.

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

How to format the year and the era. `#!typst-code "auto"`, `#!typst-code "full"`, `#!typst-code "with-era"`, `none` (default, implies `#!typst-code "auto"`).

The avialable options are also provided in [`year-styles`](./constants.md#year-styles) as a dictionary.

example{

```typst +preview(vertical)
#table(
  columns: 5,
  [`year-style`], [2024], [-128], [1984], [1847],
  ..icu
    .year-styles // (1)!
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

1. [`year-styles`](./constants.md#year-styles) is a dictionary that contains all possible values for `year-style`.

}example

### `experimental-pattern`

Specifies the pattern to format that date as. This is mutually exclusive with all other named arguments except [`zone`](#zone) and [`locale`](#locale).

<!-- prettier-ignore-->
!!! warning
    This argument is experimental. The calendar selection is implemented manually due to missing functionality in ICU4X (It's in my backlog to try and add it there).
    **This is a low-level utility that assumes the pattern is already localized for the target locale.**

The full list of placeholders can be found in the [Date Field Symbol Table]. Note that this argument doesn't check that the date and time are fully specified. If some fields are left out, they're default initialized.

The following symbols are unsupported by ICU4X: `Y+` (year in "week of year"), `u+`, `Q+`, `q+`, `w+`, `W+`, `g+`, `e`/`ee`/`c`/`cc` (numeric week), `B+`, `k+`, `j+`, `J+`, `C+`, `S+`, and `VV`.

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
#let tz = (offset: "+09", iana: "Pacific/Palau")
#let f(l, pat) = icu.fmt(dt, zone: tz, locale: l, experimental-pattern: pat)

+ #f("en", "yyyy.MM.dd G 'at' HH:mm:ss zzz")
+ #f("th-TH", "EEE, MMM d, ''yy")
+ #f("en", "h:mm a")
+ #f("nl", "hh 'o''clock' a, zzzz")
+ #f("gn", "K:mm a, z")
+ #f("en-AF", "yyyyy.MMMM.dd GGG hh:mm aaa")
+ #f("en", "y-MM-dd hh:mm a zzzz")
```

}example

[datetime]: https://typst.app/docs/reference/foundations/datetime/
[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
[Date Field Symbol Table]: https://unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
