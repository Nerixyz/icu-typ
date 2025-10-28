#import "../typst/api.typ" as icu

#{
  assert.eq(icu.fmt((:), zone: (offset: 8)), "GMT+0:00:08")
  assert.eq(icu.fmt((:), zone: (offset: 8 * 60)), "GMT+0:08")
  assert.eq(icu.fmt((:), zone: (offset: 8 * 60 * 60)), "GMT+8")
  assert.eq(icu.fmt((:), zone: (offset: -8 * 60 * 60)), "GMT-8")
  assert.eq(icu.fmt((:), zone: (offset: "-08")), "GMT-8")
  assert.eq(icu.fmt((:), zone: (offset: "-08:30")), "GMT-8:30")

  assert.eq(icu.fmt((:), zone-style: "localized-offset-long", zone: (offset: 8)), "GMT+00:00:08")
  assert.eq(icu.fmt((:), zone-style: "localized-offset-long", zone: (offset: 8 * 60)), "GMT+00:08")
  assert.eq(icu.fmt((:), zone-style: "localized-offset-long", zone: (offset: 8 * 60 * 60)), "GMT+08:00")
  assert.eq(icu.fmt((:), zone-style: "localized-offset-long", zone: (offset: -8 * 60 * 60)), "GMT-08:00")
  assert.eq(icu.fmt((:), zone-style: "localized-offset-long", zone: (offset: "-08")), "GMT-08:00")
  assert.eq(icu.fmt((:), zone-style: "localized-offset-long", zone: (offset: "-08:30")), "GMT-08:30")

  assert.eq(
    icu.fmt((:), zone: (offset: "+09:30", iana: "Australia/South"), zone-style: "specific-long"),
    "Australian Central Standard Time",
  )
  // no date -> no metazone
  assert.eq(
    icu.fmt((:), zone: (offset: "+10:30", iana: "Australia/South"), zone-style: "specific-long"),
    "Australian Central Daylight Time",
  )
  // now we got it
  let date = (year: 2020, month: 1, day: 1)
  assert.eq(
    icu.fmt(date, zone: (offset: "+10:30", iana: "Australia/South"), zone-style: "specific-long"),
    "Australian Central Daylight Time",
  )

  assert.eq(
    icu.fmt(date, zone: (offset: "+10:30", iana: "Australia/South"), zone-style: "specific-short"),
    "GMT+10:30",
  )
  assert.eq(
    icu.fmt(date, zone: (offset: "+10:30", iana: "Australia/South"), zone-style: "specific-short", locale: "en-AU"),
    "ACDT",
  )
  assert.eq(
    icu.fmt(date, zone: (offset: "+09:30", iana: "Australia/South"), zone-style: "specific-short", locale: "en-AU"),
    "ACST",
  )

  assert.eq(
    icu.fmt(date, zone: (offset: "+10:30", iana: "Australia/South"), zone-style: "generic-long"),
    "Australian Central Time",
  )
  assert.eq(
    icu.fmt(date, zone: (offset: "+09:30", iana: "Australia/South"), zone-style: "generic-long"),
    "Australian Central Time",
  )

  assert.eq(
    icu.fmt(date, zone: (offset: "+10:30", iana: "Australia/South"), zone-style: "generic-short"),
    "Adelaide Time",
  )
  assert.eq(
    icu.fmt(date, zone: (offset: "+10:30", iana: "Australia/South"), zone-style: "generic-short", locale: "en-AU"),
    "ACT",
  )
  assert.eq(
    icu.fmt(date, zone: (offset: "+09:30", iana: "Australia/South"), zone-style: "generic-short"),
    "Adelaide Time",
  )
  assert.eq(
    icu.fmt(date, zone: (offset: "+09:30", iana: "Australia/South"), zone-style: "generic-short", locale: "en-AU"),
    "ACT",
  )

  assert.eq(
    icu.fmt(date, zone: (offset: "+10:30", iana: "Australia/South"), zone-style: "exemplar-city"),
    "Adelaide",
  )
  assert.eq(
    icu.fmt(date, zone: (offset: "+09:30", iana: "Australia/South"), zone-style: "exemplar-city"),
    "Adelaide",
  )

  assert.eq(
    icu.fmt(date, zone: (offset: "+10:30", iana: "Australia/South"), zone-style: "location"),
    "Adelaide Time",
  )
  assert.eq(
    icu.fmt(date, zone: (offset: "+09:30", iana: "Australia/South"), zone-style: "location"),
    "Adelaide Time",
  )
}
