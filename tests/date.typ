#import "../typst/api.typ" as icu

#{
  let unix0 = (year: 1970, month: 1, day: 1)
  assert.eq(icu.fmt(unix0), "Jan 1, 1970")
  assert.eq(icu.fmt(unix0, date-fields: "D"), "1")
  assert.eq(icu.fmt(unix0, date-fields: "D", alignment: "column"), "01")
  assert.eq(icu.fmt(unix0, date-fields: "MD"), "Jan 1")
  assert.eq(icu.fmt(unix0, date-fields: "YMD"), "Jan 1, 1970")
  assert.eq(icu.fmt(unix0, date-fields: "DE"), "1 Thu")
  assert.eq(icu.fmt(unix0, date-fields: "MDE"), "Thu, Jan 1")
  assert.eq(icu.fmt(unix0, date-fields: "YMDE"), "Thu, Jan 1, 1970")
  assert.eq(icu.fmt(unix0, date-fields: "E"), "Thu")
  assert.eq(icu.fmt(unix0, date-fields: "M"), "Jan")
  assert.eq(icu.fmt(unix0, date-fields: "YM"), "Jan 1970")
  assert.eq(icu.fmt(unix0, date-fields: "Y"), "1970")
}

#{
  let unix0 = (year: 1970, month: 1, day: 1)
  assert.eq(icu.fmt(unix0), "Jan 1, 1970")
  assert.eq(icu.fmt(unix0, date-fields: "YMDE"), "Thu, Jan 1, 1970")
  assert.eq(icu.fmt(unix0, date-fields: "YMDE", length: "long"), "Thursday, January 1, 1970")
  assert.eq(icu.fmt(unix0, date-fields: "YMDE", length: "medium"), "Thu, Jan 1, 1970")
  assert.eq(icu.fmt(unix0, date-fields: "YMDE", length: "short"), "Thu, 1/1/70")
  assert.eq(icu.fmt((year: 2025, month: 9, day: 13), date-fields: "YMDE", length: "short"), "Sat, 9/13/25")
  assert.eq(
    icu.fmt((year: 2025, month: 9, day: 13), date-fields: "YMDE", length: "short", locale: "en-GB"),
    "Sat, 13/09/2025",
  )
  assert.eq(
    icu.fmt((year: 2025, month: 9, day: 13), date-fields: "YMDE", length: "short", locale: "de"),
    "Sa., 13.09.25",
  )
}

#{
  let y1970 = (year: 1970, month: 1, day: 1)
  let y1870 = (year: 1870, month: 1, day: 1)
  let yn70 = (year: -70, month: 1, day: 1)

  let test(dt, opts) = {
    for (key, value) in opts {
      assert.eq(icu.fmt(dt, date-fields: "Y", year-style: key, length: "long"), value.at(0))
      assert.eq(icu.fmt(dt, date-fields: "Y", year-style: key, length: "medium"), value.at(1))
      assert.eq(icu.fmt(dt, date-fields: "Y", year-style: key, length: "short"), value.at(2))
    }
  }
  test(y1970, (
    "auto": ("1970", "1970", "70"),
    "full": ("1970", "1970", "1970"),
    "with-era": ("1970 AD", "1970 AD", "1970 AD"),
  ))
  test(y1870, (
    "auto": ("1870", "1870", "1870"),
    "full": ("1870", "1870", "1870"),
    "with-era": ("1870 AD", "1870 AD", "1870 AD"),
  ))
  test(yn70, (
    "auto": ("71 BC", "71 BC", "71 BC"),
    "full": ("71 BC", "71 BC", "71 BC"),
    "with-era": ("71 BC", "71 BC", "71 BC"),
  ))
}
