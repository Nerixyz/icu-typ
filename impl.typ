#let plug = plugin("icu-datetime.wasm")

/// Creates a dictionary from a datetime or echos a dictionary passed as `dt`.
#let datetime-to-dict(dt) = {
  if type(dt) == datetime {
    (
      year: dt.year(),
      month: dt.month(),
      day: dt.day(),
      hour: dt.hour(),
      minute: dt.minute(),
      second: dt.second(),
    )
  } else if type(dt) == dictionary {
    dt
  } else {
    panic("Invalid datetime specification - expected type datetime or dictionary - got " + type(dt))
  }
}

/// Formats a date, time, or timezone.
///
/// If `date-fields`, `time-precision`, and `zone-style` all use their default values (`auto`),
/// then the format will be automatically selected based on the provided `dt` and `zone`:
/// - If `dt` has date fields, then `date-fields` will be set to "YMD"
/// - If `dt` has time fields, then `time-precision` will be set to "minute"
/// - If `zone` has a value, then `zone-style` will be set to "localized-offset-short"
///
/// - dt (dictionary, datetime): The date and time to format. This can be a `datetime` or a dictionary with `year`, `month`, `day`, `hour`, `minute`, `second`, and (optionally) `nanosecond`.
/// - zone (dictionary, none): The timezone. A dictionary with `offset`, `iana`, `bcp47`, `metazone-id`, and `zone-variant`. The options correspond to the arguments for `fmt-timezone`. Only `offset` is mandatory - the other fields provide supplemental information for named timezones.
/// - locale (str): A Unicode Locale Identifier (see https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier)
/// - length (str, none): The length of the formatted date part ("long", "medium" (default), "short", or `none`). The avialable options are also provided in `length` as a dictionary.
/// - date-fields (str, none, auto): The fields of the date to include in the formatted string. "D" (day of month), "MD", "YMD", "DE", "MDE", "YMDE", "E" (weekday), "M" (month), "YM", "Y" (year), `none`, or `auto` (default, see function documentation).
/// - time-precision (str, none, auto): How precise to display the time. "hour", "minute", "second", "subsecond{n}" (n subsecond digits), "minute-optional" ("hour" if `minutes == 0`, otherwise "minute"), `none`, or `auto` (default, see function documentation).
/// - zone-style (str, none): How to format the timezone (if any). "specific-long", "specific-short", "localized-offset-long", "localized-offset-short",  "generic-long", "generic-short", "location", "exemplar-city", `none`, or `auto` (default, see function documentation).
/// - alignment (str, none): How to align (pad) the formatted string. "auto", "column", or `none` (default, implies "auto").
/// - year-style (str, none): How to format the year and the era. "auto", "full", "with-era", `none` (default, implies "auto").
/// - experimental-pattern (str, none): Specifies the pattern to format that date as. This is mutually exclusive with all other named arguments except `zone` and `locale`. This argument is experimental. The calendar selection is implemented manually due to missing functionality in ICU4X. **This is a low-level utility that assumes the pattern is already localized for the target locale.** The full list of placeholders can be found on https://unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table. Note that this argument doesn't check that the date and time are fully specified. If some fields are left out, they're default initialized.
#let fmt(
  dt,
  zone: none,
  locale: "en",
  length: none,
  date-fields: auto,
  time-precision: auto,
  zone-style: auto,
  alignment: none,
  year-style: none,
  experimental-pattern: none,
) = {
  assert(type(locale) == str)

  let spec = datetime-to-dict(dt)
  if zone != none {
    spec.insert("zone", zone)
  }

  if experimental-pattern != none {
    return str(plug.format_pattern(cbor.encode(spec), bytes(locale), bytes(experimental-pattern)))
  }

  // only pick a format if all three are `auto`
  if date-fields == auto and time-precision == auto and zone-style == auto {
    let has-date = (
      spec.at("year", default: none) != none
        and spec.at("month", default: none) != none
        and spec.at("day", default: none) != none
    )
    let has-time = (
      spec.at("hour", default: none) != none
        and spec.at("minute", default: none) != none
        and spec.at("second", default: none) != none
    )
    let has-zone = zone != none

    if has-date {
      date-fields = "YMD"
    }
    if has-time {
      time-precision = "minute"
    }
    if has-zone {
      zone-style = "localized-offset-short"
    }
  }

  if date-fields == auto {
    date-fields = none
  }
  if time-precision == auto {
    time-precision = none
  }
  if zone-style == auto {
    zone-style = none
  }

  let opts = (
    length: length,
    date-fields: date-fields,
    time-precision: time-precision,
    zone-style: zone-style,
    alignment: alignment,
    year-style: year-style,
  )
  str(plug.format(cbor.encode(spec), bytes(locale), cbor.encode(opts)))
}

/// Gets information about ICU4X' understanding of the `locale`
///
/// `locale`: A Unicode Locale Identifier (see https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier)
#let locale-info(locale) = {
  assert(type(locale) == str)

  cbor(plug.locale_info(bytes(locale)))
}

/// Styles to format a time zone.
///
/// Note that both the offset and a time zone name (IANA or BCP47) must be given.
///
/// ```
/// #icu.fmt(
///   datetime.today(),
///   zone: (offset: "+01", iana: "Europe/Berlin"),
///   zone-style: icu.zone-styles.specific-long
/// ) // Central European Standard Time
/// ```
///
/// - specific-long: The long specific non-location format, as in "Pacific Daylight Time".
/// - specific-short: The short specific non-location format, as in "PDT".
/// - localized-offset-long: The long offset format, as in "GMT−8:00".
/// - localized-offset-short: The short offset format, as in "GMT−8".
/// - generic-long: The long generic non-location format, as in "Pacific Time".
/// - generic-short: The short generic non-location format, as in "PT".
/// - location: The location format, as in "Los Angeles time".
/// - exemplar-city: The exemplar city format, as in "Los Angeles".
#let zone-styles = (
  specific-long: "specific-long",
  specific-short: "specific-short",
  localized-offset-long: "localized-offset-long",
  localized-offset-short: "localized-offset-short",
  generic-long: "generic-long",
  generic-short: "generic-short",
  location: "location",
  exemplar-city: "exemplar-city",
)

/// The length of the formatted date/time.
///
/// - long: A long date; typically spelled-out, as in "January 1, 2000".
/// - medium: A medium-sized date; typically abbreviated, as in "Jan. 1, 2000".
/// - short: A short date; typically numeric, as in "1/1/2000".
#let length = (
  long: "long",
  medium: "medium",
  short: "short",
)

/// Fields of the date to include.
///
/// - D: Day of the month
/// - E: Day of the week
/// - M: Month
/// - Y: Year
#let fields = (
  D: "D",
  MD: "MD",
  YMD: "YMD",
  DE: "DE",
  MDE: "MDE",
  YMDE: "YMDE",
  E: "E",
  M: "M",
  YM: "YM",
  Y: "Y",
)

/// How precise the time should be included.
///
/// - hour: Only show the hour.
/// - minute: Show the hour and minute.
/// - second: Show hour, minute, and second.
/// - subsecond{n}: Show n fractional digits for the seconds.
/// - minute-optional: Show the hour and add the minute if it's non-zero.
#let time-precision = (
  hour: "hour",
  minute: "minute",
  second: "second",
  subsecond1: "subsecond1",
  subsecond2: "subsecond2",
  subsecond3: "subsecond3",
  subsecond4: "subsecond4",
  subsecond5: "subsecond5",
  subsecond6: "subsecond6",
  subsecond7: "subsecond7",
  subsecond8: "subsecond8",
  subsecond9: "subsecond9",
  minute-optional: "minute-optional",
)

/// How the numbers should be aligned.
///
/// - auto: Use locale specific alignment.
/// - column: Align the values for a column layout (i.e. pad with fields if necessary).
#let alignment = (
  auto_: "auto",
  column: "column",
)

/// How the year should be displayed.
///
/// - auto: Display the century and/or era when needed to disambiguate.
/// - full: Always display the century, and display the era when needed to disambiguate.
/// - with-era: Always display the century and era.
#let year-style = (
  auto_: "auto",
  full: "full",
  with-era: "with-era",
)
