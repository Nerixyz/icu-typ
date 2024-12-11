#import "../api.typ" as icu

#set page(width: auto, height: auto, margin: 1em)
#set text(
  font: (
    "Libertinus Serif",
    "New Computer Modern",
    "New Computer Modern Math",
    "DejaVu Sans Mono",
    "Noto Serif SC",
  ),
)

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

= Date
#icu.fmt(day, locale: "km", date-fields: "YMDE") \
#icu.fmt(day, locale: "af", date-fields: "YMDE") \
#icu.fmt(day, locale: "za", date-fields: "YMDE") \

= Time
#icu.fmt(time, locale: "id", time-precision: "second") \
#icu.fmt(time, locale: "en", time-precision: "second") \
#icu.fmt(time, locale: "ga", time-precision: "second") \

= Date and Time
#icu.fmt(dt, locale: "ru", length: "long") \
#icu.fmt(dt, locale: "en-US", length: "long") \
#icu.fmt(dt, locale: "zh-Hans-CN", length: "long") \
#icu.fmt(dt, locale: "ar", length: "long") \
#icu.fmt(dt, locale: "fi", length: "long")

= Timezone
#icu.fmt(
  datetime.today(),
  zone: tz,
  zone-style: "specific-long",
) \
#icu.fmt(
  datetime.today(),
  zone: tz,
  zone-style: "generic-short",
)

= Zoned Datetime
#let opts = (
  zone: tz,
  date-fields: "YMDE",
  time-precision: "second",
  length: "long",
)

#icu.fmt(dt, ..opts, zone-style: "generic-short") \
#icu.fmt(dt, ..opts, zone-style: "localized-offset-short", locale: "lv") \
#icu.fmt(dt, ..opts, zone-style: "exemplar-city", locale: "en-CA-u-hc-h24-ca-buddhist")

// the default undefined language
#assert.eq(
  icu.locale-info("und"),
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
    ),
  ),
)

// full unicode language identifier
#assert.eq(
  icu.locale-info("en-arab-DE-posix-macos-u-foo-bar-hc-h12-ca-buddhist-t-en-latn-US-windows-rusty-h0-hybrid-a-other-ext-x-typst-wasm"),
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
  ),
)
