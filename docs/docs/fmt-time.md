# `fmt-time`

```typst-code
let fmt-time(
  dt,
  locale: "en",
  length: "short"
)
```

Formats a time in some [`locale`](#locale).

## Arguments

### `dt`

The time to format. This can be a [`datetime`][datetime] or a dictionary with `hour`, `minute`, `second`.

example{

```typst +preview
#fmt-time(datetime(
    hour: 13,
    minute: 5,
    second: 0,
)) \
#fmt-time(( // (1)!
    hour: 14,
    minute: 53,
    second: 0,
), locale: "be")
```

1. Time passed as a dictionary

}example

### `locale`

The locale to use when formatting the time. A [Unicode Locale Identifier].

example{

```typst +preview
#let f(locale) = fmt-time(
  (hour: 13, minute: 5, second: 23),
  locale: locale,
  length: "medium",
) // (1)!

- #f("it")
- #f("ar")
- #f("en-u-hc-h12")
- #f("en-u-hc-h24")
- #f("ne")
- #f("ms")
- #f("de")
```

1. Wrapper to set the length

}example

### `length`

The length of the formatted time (`#!typst-code "medium"`, `#!typst-code "short"` (default)).

example{

```typst +preview
#let time = (
  hour: 13,
  minute: 5,
  second: 23
)

- #fmt-time(time, length: "medium")
- #fmt-time(time, length: "medium", locale: "tg")
- #fmt-time(time, length: "short")
- #fmt-time(time, length: "short", locale: "gd")
```

}example

[datetime]: https://typst.app/docs/reference/foundations/datetime/
[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
