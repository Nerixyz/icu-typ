#import "../typst/api.typ" as icu

#{
  let time = (hour: 13, minute: 38, second: 19, nanosecond: 12345678)
  let date = (year: 2024, month: 5, day: 17)
  let datetime = (:..time, ..date)
  let zone = (offset: "+08", bcp47: "phmnl")

  assert.eq(icu.fmt(time), "1:38\u{202f}PM")
  assert.eq(icu.fmt(date), "May 17, 2024")
  assert.eq(icu.fmt(datetime), "May 17, 2024, 1:38\u{202f}PM")
  assert.eq(icu.fmt(datetime, zone: zone), "May 17, 2024, 1:38\u{202f}PM GMT+8")
  assert.eq(icu.fmt(date, zone: zone), "May 17, 2024 GMT+8")
  assert.eq(icu.fmt(time, zone: zone), "1:38\u{202f}PM GMT+8")
  assert.eq(icu.fmt((:), zone: zone), "GMT+8")
}

#{
  let time = datetime(hour: 13, minute: 38, second: 19)
  let date = datetime(year: 2024, month: 5, day: 17)
  let datetime = datetime(year: 2024, month: 5, day: 17, hour: 13, minute: 38, second: 19)
  let zone = (offset: "+08", bcp47: "phmnl")

  assert.eq(icu.fmt(time), "1:38\u{202f}PM")
  assert.eq(icu.fmt(date), "May 17, 2024")
  assert.eq(icu.fmt(datetime), "May 17, 2024, 1:38\u{202f}PM")
  assert.eq(icu.fmt(datetime, zone: zone), "May 17, 2024, 1:38\u{202f}PM GMT+8")
  assert.eq(icu.fmt(date, zone: zone), "May 17, 2024 GMT+8")
  assert.eq(icu.fmt(time, zone: zone), "1:38\u{202f}PM GMT+8")
  assert.eq(icu.fmt((:), zone: zone), "GMT+8")
}
