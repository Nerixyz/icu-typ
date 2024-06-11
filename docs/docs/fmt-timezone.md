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
  includes: ()
)
```

Formats a timezone in some [`locale`](#locale).

## Arguments

### `offset`

A `str` specifying the GMT offset as an [ISO-8601 time zone designator](https://en.wikipedia.org/wiki/ISO_8601#Time_zone_designators) (`Z`, `±hh`, `±hh:mm`, or `±hhmm`). _(required)_

example{

```typst +preview
- #fmt-timezone(offset: "-07")
- #fmt-timezone(offset: "+07")
- #fmt-timezone(offset: "-03:30")
- #fmt-timezone(offset: "+1445")
- #fmt-timezone(offset: "Z")
- #fmt-timezone(offset: "-00")
```

}example

### `iana`

Name of the IANA TZ identifier (e.g. `#!typst-code "Brazil/West"` - see [IANA](https://www.iana.org/time-zones) and [Wikipedia](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)). This is mutually exclusive with [`bcp47`](#bcp47). This identifier will be converted to a BCP-47 ID.

example{

```typst +preview linenums="1"
#let dt = (
  year: 2024, month: 5, day: 31,
  hour: 18, minute: 2, second: 23,
)

#let f(offset, iana, locale: "en") = fmt-timezone(
  offset: offset,
  iana: iana,
  zone-variant: "st",
  local-date: dt,
  includes: "specific-non-location-short",
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

### `local-date`

A local date to calculate the [`metazone-id`](#metazone-id). This is mutually exclusive with [`metazone-id`](#metazone-id). When formatting [zoned-datetimes](./fmt-zoned-datetime.md) this isn't necessary. [metaZones.xml](https://github.com/unicode-org/cldr/blob/main/common/supplemental/metaZones.xml) contains a mapping of time zones to metazones at specific dates.

example{

```typst +preview
#let dt(year) = (
  year: year, month: 1, day: 1,
  hour: 12, minute: 0, second: 0,
)

#let f(iana, year) = fmt-timezone(
  offset: "Z", // not used in these cases
  iana: iana,
  zone-variant: "st",
  local-date: dt(year),
  includes: "specific-non-location-long",
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

### `zone-variant`

Many metazones use different names and offsets in the summer than in the winter. In ICU4X, this is called the _zone variant_. Supports `#!typst-code none`, `#!typst-code "st"` (standard), and `#!typst-code "dt"` (daylight).

### `locale`

The locale to use when formatting the timezone. A [Unicode Locale Identifier].

### `fallback`

The timezone format fallback. Either `#!typst-code "localized-gmt"` or a dictionary for an ISO 8601 fallback (e.g. `#!typst-code (iso8601: (format: "basic", minutes: "required", seconds: "never"))`).

### `includes`

An array or a single item (str/dictionary) of part(s) to include - corresponds to calls on [`TimeZoneFormatter`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html). Valid options are:

- `generic-location-format` (e.g. "Los Angeles Time")
- `generic-non-location-long` (e.g. "Pacific Time")
- `generic-non-location-short` (e.g. "PT")
- `localized-gmt-format` (e.g. "GMT-07:00")
- `specific-non-location-long` (e.g. "Pacific Standard Time")
- `specific-non-location-short` (e.g. "PDT")
- `iso8601`: A dictionary of ISO 8601 options `#!typst-code (iso8601: (format: "utc-basic", minutes: "optional", seconds: "optional"))` (e.g. "-07:00")

[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
