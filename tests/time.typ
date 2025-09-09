#import "../typst/api.typ" as icu

#{
  assert.eq(icu.fmt((hour: 0, minute: 0, second: 0)), "12:00\u{202f}AM")
  assert.eq(icu.fmt((hour: 0, minute: 0, second: 1)), "12:00\u{202f}AM")
  assert.eq(icu.fmt((hour: 0, minute: 1, second: 1)), "12:01\u{202f}AM")
  assert.eq(icu.fmt((hour: 1, minute: 1, second: 1)), "1:01\u{202f}AM")

  assert.eq(icu.fmt((hour: 1, minute: 1, second: 1), alignment: "column"), "01:01\u{202f}AM")
  assert.eq(icu.fmt((hour: 1, minute: 1, second: 1), time-precision: "hour"), "1\u{202f}AM")
  assert.eq(icu.fmt((hour: 1, minute: 1, second: 1), time-precision: "minute"), "1:01\u{202f}AM")
  assert.eq(icu.fmt((hour: 1, minute: 1, second: 1), time-precision: "second"), "1:01:01\u{202f}AM")
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond1"),
    "1:01:01.1\u{202f}AM",
  )
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond1"),
    "1:01:01.1\u{202f}AM",
  )
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond2"),
    "1:01:01.12\u{202f}AM",
  )
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond3"),
    "1:01:01.123\u{202f}AM",
  )
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond4"),
    "1:01:01.1234\u{202f}AM",
  )
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond5"),
    "1:01:01.12345\u{202f}AM",
  )
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond6"),
    "1:01:01.123456\u{202f}AM",
  )
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond7"),
    "1:01:01.1234567\u{202f}AM",
  )
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond8"),
    "1:01:01.12345678\u{202f}AM",
  )
  assert.eq(
    icu.fmt((hour: 1, minute: 1, second: 1, nanosecond: 123456789), time-precision: "subsecond9"),
    "1:01:01.123456789\u{202f}AM",
  )

  assert.eq(icu.fmt((hour: 0, minute: 0, second: 0), locale: "en-GB"), "00:00")
  assert.eq(icu.fmt((hour: 0, minute: 0, second: 1), locale: "en-GB"), "00:00")
  assert.eq(icu.fmt((hour: 0, minute: 1, second: 1), locale: "en-GB"), "00:01")
  assert.eq(icu.fmt((hour: 1, minute: 1, second: 1), locale: "en-GB"), "01:01")
}
