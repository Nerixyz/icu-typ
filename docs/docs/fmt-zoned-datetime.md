# `fmt-zoned-datetime` 🚧

:warning: Warning: This function is experimental and can change at any time.

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

The date and time to format. This can be a [`datetime`][datetime] or a dictionary with `year`, `month`, `day`, `hour`, `minute`, `second`, and (optionally) `nanosecond`.

### `zone`

The timezone. A dictionary with [`offset`](./fmt-timezone.md#offset), [`iana`](./fmt-timezone.md#iana), [`bcp47`](./fmt-timezone.md#bcp47), [`metazone-id`](./fmt-timezone.md#metazone-id), and [`zone-variant`](./fmt-timezone.md#zone-variant). The options correspond to the arguments for [`fmt-timezone`](./fmt-timezone.md). Only [`offset`](./fmt-timezone.md#offset) is mandatory - the other fields provide supplemental information for named timezones.

### `locale`

The locale to use when formatting the zoned datetime. A [Unicode Locale Identifier].

### `fallback`

The timezone format fallback. Either `#!typst-code "localized-gmt"` or a dictionary for an ISO 8601 fallback (e.g. `#!typst-code (iso8601: (format: "basic", minutes: "required", seconds: "never"))`).

### `date-length`

The length of the formatted date part ("full", "long" (default), "medium", "short", or `none`).

### `time-length`

The length of the formatted time part ("full", "long" (default), "medium", "short", or `none`).

[datetime]: https://typst.app/docs/reference/foundations/datetime/
[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
