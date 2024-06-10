# `fmt-datetime`

```typst-code
let fmt-datetime(
  dt,
  locale: "en",
  date-length: "long",
  time-length: "short"
)
```

Formats a date and time in some [`locale`](#locale). Dates are assumed to be ISO dates.

## Arguments

### `dt`

The date and time to format. This can be a [`datetime`][datetime] or a dictionary with `year`, `month`, `day`, `hour`, `minute`, and `second`.

example{

```typst +preview
#fmt-datetime(datetime(
  year: 2024,
  month: 5,
  day: 31,
  hour: 18,
  minute: 2,
  second: 23,
)) \
#fmt-datetime(( // (1)!
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

- #fmt-datetime(dt, locale: "en")
- #fmt-datetime(dt, locale: "ko")
- #fmt-datetime(dt, locale: "ar-u-ca-islamic")
- #fmt-datetime(dt, locale: "mk")
- #fmt-datetime(dt, locale: "so")
- #fmt-datetime(dt, locale: "fo")
- #fmt-datetime(dt, locale: "vi")
- #fmt-datetime(dt, locale: "vi-u-ca-buddhist")
```

}example

### `date-length`

The length of the formatted date part (`#!typst-code "full"`, `#!typst-code "long"` (default), `#!typst-code "medium"`, or `#!typst-code "short"`).

example{

```typst +preview(vertical)
#let dt = datetime(
  year: 2024,
  month: 5,
  day: 31,
  hour: 18,
  minute: 2,
  second: 23,
)

*Full*
- #fmt-datetime(dt, date-length: "full")
- #fmt-datetime(dt, date-length: "full", locale: "tr")
- #fmt-datetime(dt, date-length: "full", locale: "nl")

*Long*
- #fmt-datetime(dt, date-length: "long")
- #fmt-datetime(dt, date-length: "long", locale: "hi")
- #fmt-datetime(dt, date-length: "long", locale: "da")

*Medium*
- #fmt-datetime(dt, date-length: "medium")
- #fmt-datetime(dt, date-length: "medium", locale: "ja")
- #fmt-datetime(dt, date-length: "medium", locale: "pt")

*Short*
- #fmt-datetime(dt, date-length: "short")
- #fmt-datetime(dt, date-length: "short", locale: "bn")
- #fmt-datetime(dt, date-length: "short", locale: "az")
```

}example

### `time-length`

The length of the formatted time part (`#!typst-code "medium"` or `#!typst-code "short"` (default)).

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

#fmt-datetime(dt, time-length: "medium") \
#fmt-datetime(dt, time-length: "medium", locale: "fy") \
#fmt-datetime(dt, time-length: "short") \
#fmt-datetime(dt, time-length: "short", locale: "mt")
```

}example

[datetime]: https://typst.app/docs/reference/foundations/datetime/
[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
