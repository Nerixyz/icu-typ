# icu-datetime

<!-- markdownlint-disable-file MD033 -->
<!-- markdownlint-configure-file { "no-duplicate-heading": { "siblings_only": true } } -->

This library is a wrapper around [ICU4X](https://github.com/unicode-org/icu4x)' `datetime` formatting for Typst which provides internationalized formatting for dates, times, and timezones.

As the WASM bundle includes all localization data, it's quite large (about 8 MiB).

See [nerixyz.github.io/icu-typ](https://nerixyz.github.io/icu-typ) for a full API reference with more examples.

## Example

```typ
#import "@preview/icu-datetime:0.1.1": fmt-date, fmt-time, fmt-datetime, experimental
// These functions may change at any time
#import experimental: fmt-timezone, fmt-zoned-datetime

#let day = datetime(
  year: 2024,
  month: 5,
  day: 31,
)
#let time = datetime(
  hour: 18,
  minute: 2,
  second: 23,
)
#let dt = datetime(
  year: 2024,
  month: 5,
  day: 31,
  hour: 18,
  minute: 2,
  second: 23,
)
#let tz = (
  offset: "-07",
  iana: "America/Los_Angeles",
  zone-variant: "st", // standard
)

= Dates
#fmt-date(day, locale: "km", length: "full") \
#fmt-date(day, locale: "af", length: "full") \
#fmt-date(day, locale: "za", length: "full") \

= Time
#fmt-time(time, locale: "id", length: "medium") \
#fmt-time(time, locale: "en", length: "medium") \
#fmt-time(time, locale: "ga", length: "medium") \

= Date and Time
#fmt-datetime(dt, locale: "ru", date-length: "full") \
#fmt-datetime(dt, locale: "en-US", date-length: "full") \
#fmt-datetime(dt, locale: "zh-Hans-CN", date-length: "full") \
#fmt-datetime(dt, locale: "ar", date-length: "full") \
#fmt-datetime(dt, locale: "fi", date-length: "full")

= Timezones (experimental)
#fmt-timezone(
  ..tz,
  local-date: dt,
  includes: "specific-non-location-long"
) \
#fmt-timezone(
  ..tz,
  local-date: dt,
  includes: (
    iso8601: (
      format: "utc-extended",
      minutes: "required",
      seconds: "optional",
    )
  )
)

= Zoned Datetimes (experimental)
#fmt-zoned-datetime(dt, tz) \
#fmt-zoned-datetime(dt, tz, locale: "lv") \
#fmt-zoned-datetime(dt, tz, locale: "en-CA-u-hc-h24-ca-buddhist")
```

<!-- typst c res/example.typ res/example.png --root . -->

![Example](res/example.png)

Locales must be [Unicode Locale Identifier]s.
Use [`locale-info(locale)`](#locale-info) to get information on how a locale is parsed.
Unicode extensions are supported, so you can, for example, set the hour cycle with `hc-h12` or set the calendar with `ca-buddhist` (e.g. `en-CA-u-hc-h24-ca-buddhist`).

## Documentation

Documentation can be found on [nerixyz.github.io/icu-typ](https://nerixyz.github.io/icu-typ).

### `locale-info`

```typ
#let locale-info(locale)
```

Gets information about ICU4X' understanding of the `locale`

- `locale`: A [Unicode Locale Identifier]

#### Example

```typ
// the default undefined language
#assert.eq(
  locale-info("und"),
  (
    id: (
      language: "und",
      script: none,
      region: none,
      variants: (),
    ),
    extensions: (
      unicode: (keywords: "", attributes: ()),
      transform: (lang: none, fields: ""),
      private: (),
      other: (),
    )
  )
)

// full unicode language identifier
#assert.eq(
  locale-info("en-arab-DE-posix-macos-u-foo-bar-hc-h12-ca-buddhist-t-en-latn-US-windows-rusty-h0-hybrid-a-other-ext-x-typst-wasm"),
  (
    id: (
      language: "en",
      script: "Arab",
      region: "DE",
      variants: ("macos", "posix"),
    ),
    extensions: (
      unicode: (
        keywords: "ca-buddhist-hc-h12",
        attributes: ("bar", "foo"),
      ),
      transform: (
        lang: (
          language: "en",
          script: "Latn",
          region: "US",
          variants: ("rusty", "windows"),
        ),
        fields: "h0-hybrid",
      ),
      private: ("typst", "wasm"),
      other: ("a-other-ext",),
    ),
  )
)
```

## Using Locally

Download the [latest release](https://github.com/Nerixyz/icu-typ/releases), unzip it to your [local Typst packages](https://github.com/typst/packages#local-packages), and use `#import "@local/icu-datetime:0.1.1"`.

## Building

To build the library, you need to have [Rust](https://www.rust-lang.org/), [Deno](https://deno.com/), and [`wasm-opt`](https://github.com/WebAssembly/binaryen) installed.

```sh
deno task build
```

While developing, you can symlink the WASM file into the root of the repository (it's in the `.gitignore`):

```sh
# Windows (PowerShell)
New-Item icu-datetime.wasm -ItemType SymbolicLink -Value ./target/wasm32-unknown-unknown/debug/icu_typ.wasm
# Unix
ln -s ./target/wasm32-unknown-unknown/debug/icu_typ.wasm icu-datetime.wasm
```

Use `cargo b --target wasm32-unknown-unknown` to build in debug mode.

[`datetime`]: https://typst.app/docs/reference/foundations/datetime/
[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
