# `fmt-zoned-datetime` 🚧

<!-- prettier-ignore -->
!!!warning
    This function is experimental and can change at any time.

```typst-code
let fmt-zoned-datetime(
  dt,
  zone,

  locale: "en",
  fallback: "localized-gmt",
  date-length: "long",
  time-length: "long"
)
```

Formats a date and a time in a timezone. Dates are assumed to be ISO dates.

## Arguments

### `dt`

The date and time to format. This can be a [`datetime`][datetime] or a dictionary with `year`, `month`, `day`, `hour`, `minute`, and `second`.

example{

```typst +preview
#let tz = (
  offset: "-03",
  iana: "America/Thule",
  zone-variant: "dt",
)

- #fmt-zoned-datetime(datetime(
  year: 2024, month: 8, day: 15,
  hour: 13, minute: 24, second: 9
), tz)
- #fmt-zoned-datetime(( // (1)!
    year: 2024, month: 8, day: 15,
    hour: 13, minute: 24, second: 9
  ),
  tz,
  locale: "en-GB"
)
- #fmt-zoned-datetime((
    year: 2024, month: 8, day: 15,
    hour: 13, minute: 24, second: 9
  ),
  tz,
  locale: "en-GB",
  time-length: "full"
)
```

1. Datetime passed as a dictionary

}example

### `zone`

The timezone. A dictionary with [`offset`](./fmt-timezone.md#offset), [`iana`](./fmt-timezone.md#iana), [`bcp47`](./fmt-timezone.md#bcp47), [`metazone-id`](./fmt-timezone.md#metazone-id), and [`zone-variant`](./fmt-timezone.md#zone-variant). The options correspond to the arguments for [`fmt-timezone`](./fmt-timezone.md). Only [`offset`](./fmt-timezone.md#offset) is mandatory - the other fields provide supplemental information for named time zones.

[`iana`](./fmt-timezone.md#iana) and [`bcp47`](./fmt-timezone.md#bcp47) are mutually exclusive. To get the zone name, [`iana`](./fmt-timezone.md#iana)/[`bcp47`](./fmt-timezone.md#bcp47) and [`zone-variant`](./fmt-timezone.md#zone-variant) must be specified in addition to [`offset`](./fmt-timezone.md#offset). Note that not every locale has names for the zones. If a zone doesn't have a name in some locale, [`fallback`](#fallback) is used.

example{

```typst +preview
#let dt = datetime(
  year: 2024, month: 8, day: 15,
  hour: 13, minute: 24, second: 9
)

- #fmt-zoned-datetime(dt, (
  offset: "-07",
  iana: "America/Phoenix",
  zone-variant: "st",
))
- #fmt-zoned-datetime(dt, (
  offset: "+02",
  iana: "Atlantic/Jan_Mayen",
  zone-variant: "dt",
), locale: "en-GB")
- #fmt-zoned-datetime(dt, (
  offset: "+08",
  iana: "Asia/Hong_Kong",
  zone-variant: "st",
), locale: "en-HK")
- #fmt-zoned-datetime(dt, (
  offset: "+09:30",
  iana: "Australia/Adelaide",
  zone-variant: "dt",
), locale: "en-AU")
- #fmt-zoned-datetime(dt, (
  offset: "+01",
  iana: "Europe/Belfast", // (1)!
), locale: "en-GB")
- #fmt-zoned-datetime(dt, (
  offset: "Z",
))
```

1. No [`zone-variant`](./fmt-timezone.md#zone-variant) specified.

}example

### `locale`

The locale to use when formatting the zoned datetime. A [Unicode Locale Identifier]. As with [dates](./fmt-date.md#locale), `ca` can be set to a [bcp47 calendar name](https://github.com/unicode-org/cldr/blob/main/common/bcp47/calendar.xml).

example{

```typst +preview
#let dt = datetime(
  year: 2024, month: 8, day: 15,
  hour: 13, minute: 24, second: 9
)

#let tz = (
  offset: "+03",
  iana: "Asia/Nicosia",
  zone-variant: "dt",
)

- #fmt-zoned-datetime(dt, tz)
- #fmt-zoned-datetime(
    dt, tz,
    locale: "en-GB"
  )
- #fmt-zoned-datetime(
    dt, tz,
    locale: "el-CY"
  )
- #fmt-zoned-datetime(
    dt, tz,
    locale: "en-GB-u-ca-persian"
  )
- #fmt-zoned-datetime(
    dt, tz,
    locale: "en-GB-u-ca-islamic-hc-h12"
  )
```

}example

### `fallback`

The timezone format fallback. Either `#!typst-code "localized-gmt"` or a dictionary for an [ISO 8601 fallback](./fmt-timezone.md#iso-8601). This has the same effect as [`fallback`](./fmt-timezone.md#fallback) does for [`fmt-timezone`](./fmt-timezone.md).

example{

```typst +preview
#let dt = datetime(
  year: 2024, month: 8, day: 15,
  hour: 13, minute: 24, second: 9
)

#let f(
  offset,
  iso: none,
  minutes: true,
  seconds: false,
  locale: "en"
) = fmt-zoned-datetime(
  dt,
  (offset: offset), // timezone
  fallback: if iso != none {(
      iso8601: (
        format: iso,
        minutes: if minutes {
          "required"
        } else {
          "optional"
        },
        seconds: if seconds {
          "optional"
        } else {
          "never"
        },
      )
  )} else {
    "localized-gmt"
  },
  locale: locale,
)

- #f("-07")
- #f("-07", locale: "cs")
- #f("-07", locale: "da")
- #f("-07", iso: "basic")
- #f("-07", iso: "extended")
- #f("-07", iso: "utc-basic")
- #f("-07", iso: "utc-extended")
\
- #f("Z", iso: "basic")
- #f("Z", iso: "extended")
- #f("Z", iso: "utc-basic")
- #f("Z", iso: "utc-extended")
\
// 2h 15min 45s
#let sec = (2 * 60 * 60) + (15 * 60) + 45
- #f(sec, iso: "extended")
- #f(sec, iso: "extended", seconds: true)
- #f(sec, iso: "extended", minutes: false)
- #f(2 * 60 * 60, iso: "extended", minutes: false)
```

}example

### `date-length`

The length of the formatted date part (`#!typst-code "full"`, `#!typst-code "long"` (default), `#!typst-code "medium"`, `#!typst-code "short"`, or `#!typst-code  none`).

example{

```typst +preview(vertical)
#let dt = datetime(
  year: 2024, month: 8, day: 15,
  hour: 13, minute: 24, second: 9
)

#let tz = (
  offset: "+08",
  iana: "Asia/Makassar",
  zone-variant: "st",
)

#let f(
  date-length,
  locale: "en"
) = fmt-zoned-datetime(
  dt, tz,
  date-length: date-length,
  locale: locale
)

*Full*
- #f("full", locale: "en-ID")
- #f("full", locale: "th")
- #f("full", locale: "id")

*Long*
- #f("long", locale: "en-ID")
- #f("long", locale: "ha")
- #f("long", locale: "lb")

*Medium*
- #f("medium", locale: "en-ID")
- #f("medium", locale: "ky")
- #f("medium", locale: "or")

*Short*
- #f("short", locale: "en-ID")
- #f("short", locale: "pa")
- #f("short", locale: "sr")

*None*
- #f(none, locale: "en-ID")
- #f(none, locale: "bg")
- #f(none, locale: "fr")
```

}example

### `time-length`

The length of the formatted time part (`#!typst-code "full"`, `#!typst-code "long"` (default), `#!typst-code "medium"`, `#!typst-code "short"`, or `#!typst-code  none`).

example{

```typst +preview(vertical)
#let dt = datetime(
  year: 2024, month: 8, day: 15,
  hour: 13, minute: 24, second: 9
)

#let tz = (
  offset: "+08",
  iana: "Asia/Makassar",
  zone-variant: "st",
)

#let f(
  time-length,
  locale: "en"
) = fmt-zoned-datetime(
  dt, tz,
  time-length: time-length,
  locale: locale
)

*Full*
- #f("full", locale: "en-ID")
- #f("full", locale: "it")
- #f("full", locale: "lo")

*Long*
- #f("long", locale: "en-ID")
- #f("long", locale: "gu")
- #f("long", locale: "qu")

*Medium*
- #f("medium", locale: "en-ID")
- #f("medium", locale: "sk")
- #f("medium", locale: "tr")

*Short*
- #f("short", locale: "en-ID")
- #f("short", locale: "wo")
- #f("short", locale: "az")

*None*
- #f(none, locale: "en-ID")
- #f(none, locale: "eu")
- #f(none, locale: "lv")
```

}example

[datetime]: https://typst.app/docs/reference/foundations/datetime/
[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
