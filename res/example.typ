#import "../api.typ": *
#import experimental: fmt-timezone, fmt-zoned-datetime

#set page(width: auto, height: auto, margin: 1em)
#set text(
  font: (
    "Linux Libertine",
    "New Computer Modern",
    "New Computer Modern Math",
    "DejaVu Sans Mono",
    "Noto Serif SC",
  )
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

= Timezone (experimental)
#fmt-timezone(
  ..tz,
  local-date: datetime.today(),
  format: "specific-non-location-long"
) \
#fmt-timezone(
  ..tz,
  format: (
    iso8601: (
      format: "utc-extended",
      minutes: "required",
      seconds: "optional",
    )
  )
)

= Zoned Datetime (experimental)
#fmt-zoned-datetime(dt, tz) \
#fmt-zoned-datetime(dt, tz, locale: "lv") \
#fmt-zoned-datetime(dt, tz, locale: "en-CA-u-hc-h24-ca-buddhist") \

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
