# Migration

## 0.1.x to 0.2.0

All comparisons and examples here are compiled with the following imports:

```typst
#import "@preview/icu-datetime:0.2.0" as icu02
#import "@preview/icu-datetime:0.1.2" as icu01
```

### `fmt-date`

```typst-code
let fmt-date(
  dt,
  locale: "en",
  length: "full"
)
```

Changing `fmt-date` to [`fmt`][fmt] will most likely result in the same output. There are a few differences, though:

- `"full"` is no longer available as a `length` option. To get the equivalent of the full length, pass `#!typst-code date-fields: "YMDE"` to [`fmt`][fmt].
- If the passed date has time fields, then the time would be shown in addition to the date. To only show the date, specify `#!typst-code date-fields: "YMD"` explicitly. See [defaults] for more information.

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

```typst-code
let fmt-time(
  dt,
  locale: "en",
  length: "short"
)
```

Change `fmt-time` to `fmt`. In the new [`fmt`][fmt], `length` won't affect the precision. Instead, [`time-precision`][time-precision] is used. If `length` was `#!typst-code "short"` (default), specify `#!typst-code time-precision: "minute"`. Otherwise, specify `#!typst-code time-precision: "second"`.

Similar to [`fmt-date`](#fmt-date): If the passed datetime has date fields and [`time-precision`][time-precision] isn't specified, the date would be displayed as well. See [defaults] for more information.

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

```typst-code
let fmt-datetime(
  dt,
  locale: "en",
  date-length: "long",
  time-length: "short"
)
```

Change `fmt-datetime` to [`fmt`] and update the parameters:

- When using default parameters (date: long, time: short), specify `#!typst-code length: "long"`.
- `date-length`
    - `#!typst-code "full"`: specify `#!typst-code date-fields: "YMDE", length: "long"`
    - `#!typst-code "long"` (old default): specify `#!typst-code date-fields: "YMD", length: "long"`
    - `#!typst-code "medium"`: specify `#!typst-code date-fields: "YMD", length: "medium"`
    - `#!typst-code "short"`: specify `#!typst-code date-fields: "YMD", length: "short"`
- `time-length`:
    - `#!typst-code "medium"`: specify `#!typst-code time-precision: "second"`
    - `#!typst-code "short"` (old default): specify `#!typst-code time-precision: "minute"`
- Make sure that either both of [`date-fields`][date-fields] and [`time-precision`][time-precision] or neither are specified (see [defaults] for more information).

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

### `fmt-timezone`

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

As with the other functions, this is replaced by [`fmt`][fmt]. Specify [`zone`][zone] (dictionary with `offset`, `iana`/`bcp47`) and [`zone-style`][zone-style] to output just the zone. Zone variants are now automatically resolved if a date is given. Otherwise, the standard variant is always used. The `fallback` is no longer customizable.

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

### `fmt-zoned-datetime`

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

This is most similar to the new [`fmt`][fmt]. After changing the call itself, consider the following changes to the arguments:

- `zone`: This was previously a positional argument. In [`fmt`][fmt], it's now a named one (see [`zone`][zone]). In the dictionary itself, `metazone-id` and `zone-variant` have been removed. The zone variant is now automatically inferred from the date.
- `fallback`: This argument was removed. The fallback is always localized-gmt (i.e. ` GMT±hh:mm`).
- `date-length`: Same changes as for [`fmt-datetime`](#fmt-datetime) are required. For `none`, pass `none` to [`date-fields`][date-fields].
- `time-length`: This mostly corresponds to [`time-precision`][time-precision].
    - `#!typst-code "full"`: Pass `#!typst-code time-precision: "second", zone-style: "specific-long"`
    - `#!typst-code "long"`: Pass `#!typst-code time-precision: "second", zone-style: "specific-short"`
    - `#!typst-code "medium"`: Pass `#!typst-code time-precision: "second"`
    - `#!typst-code "short"`: Pass `#!typst-code time-precision: "minute"`
    - `#!typst-code none`: Pass `#!typst-code time-precision: none`

example(Comparison){

_Note:_ This only shows a comparison for `time-length`, as the changes to `date-length` are equivalent to [`fmt-datetime`](#fmt-datetime).

```typst +preview(01-to-02, vertical)
#let dt = (
  year: 2024, month: 6, day: 30,
  hour: 18, minute: 2, second: 23,
)
#let zone01 = (
  offset: "-07",
  iana: "America/Los_Angeles",
  // in 0.1.2, we had to specify the variant explcicitly
  zone-variant: "dt",
)
#let zone02 = (offset: "-07", iana: "US/Pacific")

#let fmt-zoned-dt01 = icu01.experimental.fmt-zoned-datetime
#table(
  columns: 3,
  [`time-length` (0.1.2)], [*0.1.2*], [*0.2.0*],

  [full],
  fmt-zoned-dt01(dt, zone01, time-length: "full", date-length: none),
  icu02.fmt(
    dt,
    zone: zone02,
    time-precision: "second",
    zone-style: "specific-long",
  ),

  [long (default)],
  fmt-zoned-dt01(dt, zone01, time-length: "long", date-length: none),
  icu02.fmt(
    dt,
    zone: zone02,
    time-precision: "second",
    zone-style: "specific-short",
  ),

  [medium],
  fmt-zoned-dt01(dt, zone01, time-length: "medium", date-length: none),
  icu02.fmt(dt, zone: zone02, time-precision: "second"),

  [short],
  fmt-zoned-dt01(dt, zone01, time-length: "short", date-length: none),
  icu02.fmt(dt, zone: zone02, time-precision: "minute"),
)
```

}

[fmt]: ./fmt.md
[zone]: ./fmt.md#zone
[zone-style]: ./fmt.md#zone-style
[time-precision]: ./fmt.md#time-precision
[date-fields]: ./fmt.md#date-fields
[defaults]: ./fmt.md#defaults
