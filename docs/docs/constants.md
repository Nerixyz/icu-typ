# Constants

Many named arguments to [`fmt`][fmt] take a set of string values used similar to enums of other languages.
The package also provides dictionaries of all accepted values for these parameters.

## `zone-styles`

```typst-code
let zone-styles = (
  /// The long specific non-location format, as in "Pacific Daylight Time".
  specific-long: "specific-long",
  /// The short specific non-location format, as in "PDT".
  specific-short: "specific-short",
  /// The long offset format, as in "GMT−8:00".
  localized-offset-long: "localized-offset-long",
  /// The short offset format, as in "GMT−8".
  localized-offset-short: "localized-offset-short",
  /// The long generic non-location format, as in "Pacific Time".
  generic-long: "generic-long",
  /// The short generic non-location format, as in "PT".
  generic-short: "generic-short",
  /// The location format, as in "Los Angeles time".
  location: "location",
  /// The exemplar city format, as in "Los Angeles".
  exemplar-city: "exemplar-city",
)
```

Corresponds to the [`zone-style`](./fmt.md#zone-style) argument of [`fmt`][fmt].

Note that both the offset and a time zone name (IANA or BCP47) must be given to [`zone`](./fmt.md#zone).

```typst +preview
#icu.fmt(
  // to resolve the zone variant for "+01"
  datetime.today(),
  zone: (offset: "+01", iana: "Europe/Berlin"),
  zone-style: icu.zone-styles.specific-long
)
```

## `length`

```typst-code
let length = (
  /// A long date; typically spelled-out, as in "January 1, 2000".
  long: "long",
  /// A medium-sized date; typically abbreviated, as in "Jan. 1, 2000".
  medium: "medium",
  /// A short date; typically numeric, as in "1/1/2000".
  short: "short",
)
```

Corresponds to the [`length`](./fmt.md#length) argument of [`fmt`][fmt].

## `date-fields`

```typst-code
let date-fields = (
  D: "D", // day of the month
  MD: "MD",
  YMD: "YMD",
  DE: "DE",
  MDE: "MDE",
  YMDE: "YMDE",
  E: "E", // day of the week
  M: "M", // month
  YM: "YM",
  Y: "Y", // year
)
```

Corresponds to the [`date-fields`](./fmt.md#date-fields) argument of [`fmt`][fmt].

## `time-precision`

```typst-code
#let time-precision = (
  /// Only show the hour.
  hour: "hour",
  /// Show the hour and minute.
  minute: "minute",
  /// Show hour, minute, and second.
  second: "second",
  /// Show n fractional digits for the seconds.
  subsecond1: "subsecond1",
  subsecond2: "subsecond2",
  subsecond3: "subsecond3",
  subsecond4: "subsecond4",
  subsecond5: "subsecond5",
  subsecond6: "subsecond6",
  subsecond7: "subsecond7",
  subsecond8: "subsecond8",
  subsecond9: "subsecond9",
  /// Show the hour and add the minute if it's non-zero.
  minute-optional: "minute-optional",
)
```

Corresponds to the [`time-precision`](./fmt.md#time-precision) argument of [`fmt`][fmt].

## `alignment`

```typst-code
let alignment = (
  /// Use locale specific alignment.
  auto_: "auto",
  /// Align the values for a column layout (i.e. pad with fields if necessary).
  column: "column",
)
```

Corresponds to the [`alignment`](./fmt.md#alignment) argument of [`fmt`][fmt].

## `year-styles`

```typst-code
let year-styles = (
  /// Display the century and/or era when needed to disambiguate.
  auto_: "auto",
  /// Always display the century, and display the era when needed to disambiguate.
  full: "full",
  /// Always display the century and era.
  with-era: "with-era",
)
```

Corresponds to the [`year-style`](./fmt.md#year-style) argument of [`fmt`][fmt].

[fmt]: ./fmt.md
