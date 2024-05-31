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

#fmt-date(day, locale: "de", length: "full") \
#fmt-time(time, locale: "de", length: "medium") \
#fmt-datetime(dt, locale: "fi", date-length: "full") \
#fmt-timezone(
  "-07",
  iana: "America/Los_Angeles",
  local-date: dt,
  zone-variant: "st",
  includes: "specific-non-location-long"
) \
#fmt-zoned-datetime(
  dt, 
  (
    offset: "-07",
    iana: "America/Los_Angeles",
    zone-variant: "st", // standard
  )
)
