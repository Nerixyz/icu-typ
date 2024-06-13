# `fmt-timezone` 🚧

<!-- prettier-ignore -->
!!!warning
    This function is experimental and can change at any time.

```typst-code
let fmt-timezone(
  offset: none, // required

  iana: none,
  bcp47: none,

  local-date: none,
  metazone-id: none,

  zone-variant: none,

  locale: "en",
  fallback: "localized-gmt",
  format: none
)
```

Formats a timezone in some [`locale`](#locale).

## Arguments

### `offset`

A `str` specifying the GMT offset as an [ISO-8601 time zone designator](https://en.wikipedia.org/wiki/ISO_8601#Time_zone_designators) (`Z`, `±hh`, `±hh:mm`, or `±hhmm`). Since v0.1.2, this can be an `int` which specifies the offset in seconds. _(required)_

example{

```typst +preview
- #fmt-timezone(offset: "-07")
- #fmt-timezone(offset: "+07")
- #fmt-timezone(offset: "-03:30")
- #fmt-timezone(offset: "+1445")
- #fmt-timezone(offset: "Z")
- #fmt-timezone(offset: "-00")
// v0.1.2 and later:
- #fmt-timezone(offset: 60 * 60)
- #fmt-timezone(offset: 30 * 60)
- #fmt-timezone(offset: 30 * 60 + 30) // (1)!
```

1. See [`iso8601` fallback format](#fallback).

}example

### `iana`

Name of the IANA TZ identifier (e.g. `#!typst-code "Brazil/West"` - see [IANA](https://www.iana.org/time-zones) and [Wikipedia](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)). This is mutually exclusive with [`bcp47`](#bcp47). This identifier will be converted to a BCP-47 ID.

example{

```typst +preview linenums="1"
#let f(offset, iana, locale: "en") = fmt-timezone(
  offset: offset,
  iana: iana,
  zone-variant: "st",
  local-date: datetime.today(),
  format: "specific-non-location-short",
  locale: locale
)
#set enum(start: 16)

+ #f("-06", "Canada/Central", locale: "fi")
+ #f("-05", "Canada/Eastern")
+ #f("-04", "Canada/Atlantic")
+ #f("+02", "Libya", locale: "en-GB")
+ #f("+02", "Africa/Windhoek", locale: "en-GB")
+ #f("+02", "Africa/Windhoek", locale: "en-NA")
+ #f("+02", "Africa/Johannesburg", locale: "en-NA")
+ #f("+03", "Indian/Antananarivo", locale: "af")
+ #f("+07", "Asia/Jakarta", locale: "id")
+ #f("+0930", "Australia/Adelaide", locale: "en-AU")
```

}example

### `bcp47`

Name of the BCP-47 timezone ID (e.g. `#!typst-code "iodga"` - see [timezone.xml](https://github.com/unicode-org/cldr/blob/main/common/bcp47/timezone.xml)). This is mutually exclusive with [`iana`](#iana).

<!-- prettier-ignore -->
!!! warning
    In v0.1.1 it's not possible to set the BCP-47 timezone ID - use [`iana`](#iana) instead. This will be fixed in v0.1.2.

example{

```typst +preview linenums="1"
#let f(offset, bcp47) = fmt-timezone(
  offset: offset,
  bcp47: bcp47,
  zone-variant: "st",
  local-date: datetime.today(),
  format: "specific-non-location-long",
  locale: "en"
)
#set enum(start: 16)

+ #f("+01", "debsngn")
+ #f("+07", "khpnh")
+ #f("+12", "mhmaj")
+ #f("+01", "nenim")
+ #f("-08", "pst8pdt")
+ #f("+09", "ruchita")
+ #f("+11", "sbhir")
+ #f("+12", "tvfun")
+ #f("+05", "invalid")
```

}example

### `local-date`

A local date to calculate the [`metazone-id`](#metazone-id). This is mutually exclusive with [`metazone-id`](#metazone-id). This can be a dictionary or a [`datetime`](https://typst.app/docs/reference/foundations/datetime) with or without time (`hour`, `minute`, `second` - these will be 0 by default).
When formatting [zoned-datetimes](./fmt-zoned-datetime.md) this isn't necessary. [metaZones.xml](https://github.com/unicode-org/cldr/blob/main/common/supplemental/metaZones.xml) contains a mapping of time zones to metazones at specific dates.

example{

```typst +preview
#let dt(year) = (
  year: year, month: 1, day: 1,
  hour: 12,
  // minute and second default to 0
)

#let f(iana, year) = fmt-timezone(
  offset: "Z", // not used in these cases
  iana: iana,
  zone-variant: "st",
  local-date: dt(year),
  format: "specific-non-location-long",
)

- #f("Africa/Tripoli", 1981)
- #f("Africa/Tripoli", 1982)
- #f("Africa/Tripoli", 1991)
\
- #f("Europe/Vilnius", 1997)
- #f("Europe/Vilnius", 1999)
\
- #f("Pacific/Galapagos", 1985)
- #f("Pacific/Galapagos", 1986)
```

}example

### `metazone-id`

A short ID of the metazone. A metazone is a collection of multiple time zones that share the same localized formatting at a particular date and time (e.g. `#!typst-code "phil"` - see [metaZones.xml](https://github.com/unicode-org/cldr/blob/main/common/supplemental/metaZones.xml) (bottom)).

example{

```typst +preview linenums="1"
#let f(metazone-id) = fmt-timezone(
  offset: "Z", // (1)!
  zone-variant: "st",
  metazone-id: metazone-id,
  format: "specific-non-location-long",
  locale: "en"
)
#set enum(start: 10)

+ #f("arge")
+ #f("chri")
+ #f("dumo")
+ #f("eufe")
+ #f("haal")
+ #f("loho")
+ #f("niue")
+ #f("kosr")
+ #f("----") // invalid
```

1. `offset` doesn't need to correspond to the metazone.

}example

### `zone-variant`

Many metazones use different names and offsets in the summer than in the winter. In ICU4X, this is called the _zone variant_. Supports `#!typst-code none`, `#!typst-code "st"` (standard), and `#!typst-code "dt"` (daylight).

example{

```typst +preview(vertical)
#let f(metazone-id, variant) = fmt-timezone(
  offset: "Z", // (1)!
  zone-variant: variant,
  metazone-id: metazone-id,
  format: "specific-non-location-long",
  locale: "en"
)

#let c(metazone-id) = (
  f(metazone-id, "st"),
  f(metazone-id, "dt")
)

#table(
  columns: (auto, auto),
  table.header([*st* (standard)],[*dt* (daylight)]),
  ..c("ammo"),
  ..c("coco"), // (2)!
  ..c("euea"),
  ..c("haal"),
  ..c("loho"),
  ..c("mosc"),
  ..c("neze"),
)
```

1. `offset` doesn't need to correspond to the metazone.
2. Cocos Islands only have a single timezone (no summer/winter time).

}example

### `locale`

The locale to use when formatting the timezone. A [Unicode Locale Identifier].

example{

```typst +preview
#let f(
  locale,
  metazone-id: none,
  offset: "Z",
) = fmt-timezone(
  offset: offset,
  zone-variant: "st",
  metazone-id: metazone-id,
  format: "specific-non-location-long",
  locale: locale,
)

- #f("ko", metazone-id: "bang")
- #f("lo", metazone-id: "cook")
- #f("ms", metazone-id: "inwe")
- #f("nl", metazone-id: "peru")
- #f("en", offset: "+06")
- #f("fi", offset: "+06")
- #f("si", offset: "+06")
```

}example

### `fallback`

The timezone format fallback. Either `#!typst-code "localized-gmt"` or a dictionary for an [ISO 8601](#iso-8601) fallback (e.g. `#!typst-code (iso8601: (format: "basic", minutes: "required", seconds: "never"))`).

example{

```typst +preview
#let f(
  offset,
  iso: none,
  minutes: true,
  seconds: false,
  locale: "en"
) = fmt-timezone(
  offset: offset,
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

- #f("+06")
- #f("+06", locale: "cs")
- #f("+06", locale: "da")
- #f("+06", iso: "basic")
- #f("+06", iso: "extended")
- #f("+06", iso: "utc-basic")
- #f("+06", iso: "utc-extended")
\
- #f("Z", iso: "basic")
- #f("Z", iso: "extended")
- #f("Z", iso: "utc-basic")
- #f("Z", iso: "utc-extended")
\
// 1h 30min 30s
#let sec = 90 * 60 + 30
- #f(sec, iso: "extended")
- #f(sec, iso: "extended", seconds: true)
- #f(sec, iso: "extended", minutes: false)
- #f(60 * 60, iso: "extended", minutes: false)
```

}example

### `format`

The format to display a time zone as (see [Time Zone Format Terminology](https://unicode.org/reports/tr35/tr35-dates.html#time-zone-format-terminology)). Note that not every [`locale`](#locale) has definitions for all formats. If none is found, [`fallback`](#fallback) will be used to format the timezone. Valid options are:

- `generic-location` (e.g. "Los Angeles Time") [`bcp47`](#bcp47) or [`iana`](#iana) must be specified
- `generic-non-location-long` (e.g. "Pacific Time") [`local-date`](#local-date) or [`metazone-id`](#metazone-id) must be specified
- `generic-non-location-short` (e.g. "PT") [`local-date`](#local-date) or [`metazone-id`](#metazone-id) must be specified
- `localized-gmt` (e.g. "GMT-07:00")
- `specific-non-location-long` (e.g. "Pacific Standard Time") [`local-date`](#local-date) or [`metazone-id`](#metazone-id) must be specified
- `specific-non-location-short` (e.g. "PDT") [`local-date`](#local-date) or [`metazone-id`](#metazone-id) must be specified.
- A dictionary of [ISO 8601](#iso-8601) options (e.g. "-07:00")

example{

```typst +preview(vertical)
#let f(offset, iana, locale: "en") = (
  "generic-location",
  "generic-non-location-long",
  "generic-non-location-short",
  "localized-gmt",
  "specific-non-location-long",
  "specific-non-location-short"
).map(format => fmt-timezone(
  offset: offset,
  zone-variant: "st",
  iana: iana,
  local-date: datetime.today(),
  format: format,
  locale: locale
))

#let hc(..args) = table.cell(align: center, ..args)

#table(
  columns: (auto, auto, auto, auto, auto, auto),
  hc(rowspan: 2, align: bottom + center)[generic-location],
  hc(colspan: 2)[generic-non-location-],
  hc(rowspan: 2, align: bottom)[localized-gmt],
  hc(colspan: 2)[specific-non-location-],
  hc(x: 1)[long],
  hc[short],
  hc(x: 4)[long],
  hc[short],

  ..f("-11", "Pacific/Midway"),
  ..f("-07", "US/Pacific"),
  ..f("-06", "Mexico/General"),
  ..f("-05", "Jamaica"),
  ..f("-04", "Chile/Continental", locale: "es-CL"),
  ..f("-03:30", "Canada/Newfoundland", locale: "en-CA"),
  ..f("-03", "Brazil/East", locale: "pt-BR"),
  ..f("-02", "America/Godthab"),
  ..f("-01", "Atlantic/Azores"),
  ..f("Z", "Africa/Timbuktu"),
  ..f("+01", "Arctic/Longyearbyen", locale: "en-GB"),
  ..f("+02", "Africa/Johannesburg", locale: "en-ZA"),
  ..f("+03", "Indian/Mayotte", locale: "en-MG"),
)
```

}example

#### ISO-8601

ISO-8601 options are passed as a dictionary inside a dictionary with the `#!typst-code iso8601` key. The options must include the following keys:

- `#!typst-code format`: one of `#!typst-code "basic"`, `#!typst-code "extended"`, `#!typst-code "utc-basic"`, or `#!typst-code "utc-extended"`
- `#!typst-code minutes`: either `#!typst-code "required"` or `#!typst-code "optional"`
- `#!typst-code seconds`: either `#!typst-code "optional"` or `#!typst-code "never"`

example{

```typst +preview
#let f(
  offset,
  format,
  minutes: "required",
  seconds: "never",
) = fmt-timezone(
  offset: offset,
  format: (
      iso8601: (
        format: format,
        minutes: minutes,
        seconds: seconds,
    ),
  ),
)

- #f("-03", "basic")
- #f("-03", "extended")
- #f("-03", "utc-basic")
- #f("-03", "utc-extended")
\
- #f("Z", "basic")
- #f("Z", "extended")
- #f("Z", "utc-basic")
- #f("Z", "utc-extended")
\
// 2h 30min 30s
#let sec = 2 * 60 * 60 + 30 * 60 + 30
- #f(sec, "extended")
- #f(sec, "extended", seconds: "optional")
- #f(sec, "extended", minutes: "optional")
- #f(2 * 60 * 60, "extended", minutes: "optional")
```

}example

[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
