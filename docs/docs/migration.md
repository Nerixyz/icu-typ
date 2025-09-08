# Migration

## 0.1.x to 0.2.0

All comparisons and examples here are compiled with the following imports:

```typst
#import "@preview/icu-datetime:0.2.0" as icu02
#import "@preview/icu-datetime:0.1.2" as icu01
```

### `fmt-date`

Changing `fmt-date` to [`fmt`](./fmt.md) will most likely result in the same output. There are a few differences, though:

-   `"full"` is no longer available as a `length` option. To get the equivalent of the full length, pass `#!typst-code date-fields: "YMDE"` to [`fmt`](./fmt.md).
-   If the passed date has time fields, then the time would be shown in addition to the date. To only show the date, specify `#!typst-code date-fields: "YMD"` explicitly. See [defaults](./fmt.md#defaults) for more information.

example(Comparison){

```typst +preview(01-to-02, vertical)
#let date = (day: 12, month: 8, year: 2024)
#table(
  columns: 2,
  [*0.1.2*], [*0.2.0*],

  icu01.fmt-date(date),
  icu02.fmt(date, length: "long", date-fields: "YMDE"),

  icu01.fmt-date(date, length: "long"),
  icu02.fmt(date, length: "long"),

  icu01.fmt-date(date, length: "medium"),
  icu02.fmt(date, length: "medium"),

  icu01.fmt-date(date, length: "short"),
  icu02.fmt(date, length: "short"),
)
```

}

### `fmt-time`

Change `fmt-time` to `fmt`. In the new [`fmt`](./fmt.md), `length` won't affect the precision. Instead, [`time-precision`](./fmt.md#time-precision) is used. If `length` was `#!typst-code "short"` (default), specify `#!typst-code time-precision: "minute"`. Otherwise, specify `#!typst-code time-precision: "second"`.

Similar to [`fmt-date`](#fmt-date): If the passed datetime has date fields and [`time-precision`](./fmt.md#time-precision) isn't specified, the date would be displayed as well. See [defaults](./fmt.md#defaults) for more information.

example(Comparison){

```typst +preview(01-to-02, vertical)
#let time = (hour: 13, minute: 5, second: 0)
#table(
  columns: 2,
  [*0.1.2*], [*0.2.0*],

  icu01.fmt-time(time), icu02.fmt(time),
  icu01.fmt-time(time, length: "medium"), icu02.fmt(time, time-precision: "second"),
  icu01.fmt-time(time, length: "short"), icu02.fmt(time),
)
```

}

### `fmt-datetime`

Change `fmt-datetime` to [`fmt`](./fmt.md) and update the parameters:

-   When using default parameters (date: long, time: short), specify `#!typst-code length: "long"`.
-   `date-length`
    -   `#!typst-code "full"`: specify `#!typst-code date-fields: "YMDE", length: "long"`
    -   `#!typst-code "long"` (old default): specify `#!typst-code date-fields: "YMD", length: "long"`
    -   `#!typst-code "medium"`: specify `#!typst-code date-fields: "YMD", length: "medium"`
    -   `#!typst-code "short"`: specify `#!typst-code date-fields: "YMD", length: "short"`
-   `time-length`:
    -   `#!typst-code "medium"`: specify `#!typst-code time-precision: "second"`
    -   `#!typst-code "short"` (old default): specify `#!typst-code time-precision: "minute"`
-   Make sure that either both of [`date-fields`](./fmt.md#date-fields) and [`time-precision`](./fmt.md#time-precision) or neither are specified (see [defaults](./fmt.md#defaults) for more information).

example(Comparison){

```typst +preview(01-to-02, vertical)
#let dt = (
  year: 2024, month: 6, day: 30,
  hour: 18, minute: 2, second: 23,
)
#table(
  columns: 2, stroke: 0.5pt,
  [*0.1.2*], [*0.2.0*],
  table.hline(stroke: 2pt),

  icu01.fmt-datetime(dt), icu02.fmt(dt, length: "long"),

  icu01.fmt-datetime(dt, date-length: "full", time-length: "short"),
  icu02.fmt(dt, date-fields: "YMDE", length: "long", time-precision: "minute"),

  icu01.fmt-datetime(dt, date-length: "long", time-length: "short"),
  icu02.fmt(dt, date-fields: "YMD", length: "long", time-precision: "minute"),

  icu01.fmt-datetime(dt, date-length: "medium", time-length: "short"),
  icu02.fmt(dt, date-fields: "YMD", length: "medium", time-precision: "minute"),

  icu01.fmt-datetime(dt, date-length: "short", time-length: "short"),
  icu02.fmt(dt, date-fields: "YMD", length: "short", time-precision: "minute"),

  table.hline(stroke: 2pt),

  icu01.fmt-datetime(dt, date-length: "full", time-length: "medium"),
  icu02.fmt(dt, date-fields: "YMDE", length: "long", time-precision: "second"),

  icu01.fmt-datetime(dt, date-length: "long", time-length: "medium"),
  icu02.fmt(dt, date-fields: "YMD", length: "long", time-precision: "second"),

  icu01.fmt-datetime(dt, date-length: "medium", time-length: "medium"),
  icu02.fmt(dt, date-fields: "YMD", length: "medium", time-precision: "second"),

  icu01.fmt-datetime(dt, date-length: "short", time-length: "medium"),
  icu02.fmt(dt, date-fields: "YMD", length: "short", time-precision: "second"),
)
```

}

### `fmt-zone`

As with the other functions, this is replaced by [`fmt`](./fmt.md). Specify [`zone`](./fmt.md#zone) (dictionary with `offset`, `iana`/`bcp47`) and [`zone-style`](./fmt.md#zone-style) to output just the zone. Zone variants are now automatically resolved if a date is given. Otherwise, the standard variant is always used. The `fallback` is no longer customizable.

example(Comparison){

```typst +preview(01-to-02, vertical)
// from the examples
#table(
  columns: 2,
  [*0.1.2*], [*0.2.0*],

  icu01.experimental.fmt-timezone(
    offset: "-07",
    iana: "America/Los_Angeles",
    zone-variant: "st",
    local-date: datetime.today(),
    format: "specific-non-location-long",
  ),
  // We didn't specify a date, so the standard variant will be used.
  icu02.fmt(
    (:), // empty dictionary
    zone: (offset: "-07", iana: "America/Los_Angeles"),
    zone-style: "specific-long",
  ),

  icu01.experimental.fmt-timezone(
    offset: "-07",
    iana: "America/Los_Angeles",
    zone-variant: "st",
    format: (
      iso8601: (
        format: "utc-extended",
        minutes: "required",
        seconds: "optional",
      ),
    ),
  ),
  icu02.fmt(
    (:), // empty dictionary
    zone: (offset: "-07", iana: "America/Los_Angeles"),
    zone-style: "localized-offset-long", // GMT is always included
  ),
)
```

}
