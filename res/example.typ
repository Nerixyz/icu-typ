#import "../api.typ": *
#import experimental: fmt-timezone, fmt-zoned-datetime

#set page(width: auto, height: auto, margin: 1em)

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

= Zoned Datetime (experimental)
#fmt-zoned-datetime(dt, tz) \
#fmt-zoned-datetime(dt, tz, locale: "lv") \
#fmt-zoned-datetime(dt, tz, locale: "de") \
