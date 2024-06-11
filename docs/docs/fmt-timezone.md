# `fmt-timezone` 🚧

:warning: Warning: This function is experimental and can change at any time.

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

A string specifying the GMT offset (e.g. "-07", "Z", "+05", "+0500", "+05:00"). _(required)_

### `iana`

Name of the IANA TZ identifier (e.g. "Brazil/West" - see [IANA](https://www.iana.org/time-zones) and [Wikipedia](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)). This is mutually exclusive with [`bcp47`](#bcp47). This identifier will be converted to a BCP-47 ID.

### `bcp47`

Name of the BCP-47 timezone ID (e.g. "iodga" - see [timezone.xml](https://github.com/unicode-org/cldr/blob/main/common/bcp47/timezone.xml)). This is mutually exclusive with [`iana`](#iana).

### `local-date`

A local date to calculate the metazone-id. This is mutually exclusive with [`metazone-id`](#metazone-id). When formatting zoned-datetimes this isn't necessary.

### `metazone-id`

A short ID of the metazone. A metazone is a collection of multiple time zones that share the same localized formatting at a particular date and time (e.g. "phil" - see [metaZones.xml](https://github.com/unicode-org/cldr/blob/main/common/supplemental/metaZones.xml) (bottom)).

### `zone-variant`

Many metazones use different names and offsets in the summer than in the winter. In ICU4X, this is called the _zone variant_. Supports `none`, `#!typst-code "st"` (standard), and `#!typst-code "dt"` (daylight).

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
