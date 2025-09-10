#import "../typst/api.typ" as icu

#{
  let dt = (
    year: 2024,
    month: 7,
    day: 8,
    hour: 18,
    minute: 2,
    second: 23,
    nanosecond: 12345678,
  )
  let tz = (offset: "-08", iana: "America/Los_Angeles")
  let f(pat, locale: "en") = icu.fmt(dt, zone: tz, locale: locale, experimental-pattern: pat)

  assert.eq(f("yyyy.MM.dd G 'at' HH:mm:ss zzz"), "2024.07.08 AD at 18:02:23 PST")

  assert.eq(f("G GG GGG"), "AD AD AD")
  assert.eq(f("GGGG"), "Anno Domini")
  assert.eq(f("GGGGG"), "A")

  assert.eq(f("y yy yyy yyyy yyyyy yyyyyy"), "2024 24 2024 2024 02024 002024")

  // 'Y+' not supported by ICU4X
  assert.eq(f("Y"), "Y")
  // 'u+' not supported by ICU4X
  assert.eq(f("u"), "u")

  assert.eq(f("U UU UUU", locale: "en-u-ca-chinese"), "jia-chen jia-chen jia-chen")
  assert.eq(f("UUUU", locale: "en-u-ca-chinese"), "jia-chen")
  assert.eq(f("UUUUU", locale: "en-u-ca-chinese"), "jia-chen")

  assert.eq(f("r", locale: "en-u-ca-dangi"), "2024")
  assert.eq(f("r", locale: "en-u-ca-chinese"), "2024")

  // 'Q' not supported by ICU4X
  assert.eq(f("Q"), "Q")
  // 'q' not supported by ICU4X
  assert.eq(f("q"), "q")

  assert.eq(f("M MM MMM"), "7 07 Jul")
  assert.eq(f("MMMM"), "July")
  assert.eq(f("MMMMM"), "J")
  assert.eq(f("L LL LLL"), "7 07 Jul")
  assert.eq(f("LLLL"), "July")
  assert.eq(f("LLLLL"), "J")

  // 'w' not supported by ICU4X
  assert.eq(f("w"), "w")
  // 'W' not supported by ICU4X
  assert.eq(f("W"), "W")

  assert.eq(f("d dd"), "8 08")
  assert.eq(f("D DD DDD"), "190 190 190")

  assert.eq(f("F"), "2")

  // 'g' not supported by ICU4X
  assert.eq(f("g"), "g")

  assert.eq(f("E EE EEE"), "Mon Mon Mon")
  assert.eq(f("EEEE"), "Monday")
  assert.eq(f("EEEEE"), "M")
  assert.eq(f("EEEEEE"), "Mo")

  // 'e' and 'ee' not supported
  assert.eq(f("eee"), "Mon")
  assert.eq(f("eeee"), "Monday")
  assert.eq(f("eeeee"), "M")
  assert.eq(f("eeeeee"), "Mo")

  // 'c' and 'cc' not supported
  assert.eq(f("ccc"), "Mon")
  assert.eq(f("cccc"), "Monday")
  assert.eq(f("ccccc"), "M")
  assert.eq(f("cccccc"), "Mo")

  assert.eq(f("a aa aaa"), "PM PM PM")
  assert.eq(f("aaaa"), "PM")
  assert.eq(f("aaaaa"), "p")

  assert.eq(f("b bb bbb"), "PM PM PM")
  assert.eq(f("bbbb"), "PM")
  assert.eq(f("bbbbb"), "p")

  // 'B' not supported by ICU4X
  assert.eq(f("B"), "B")

  assert.eq(f("h hh H HH K KK"), "6 06 18 18 6 06")

  // 'k' not supported by ICU4X
  assert.eq(f("k"), "k")
  // 'j' not supported by ICU4X
  assert.eq(f("j"), "j")
  // 'J' not supported by ICU4X
  assert.eq(f("J"), "J")
  // 'C' not supported by ICU4X
  assert.eq(f("C"), "C")

  assert.eq(f("m mm"), "2 02")
  assert.eq(f("s ss"), "23 23")

  // 'S' not supported by ICU4X
  assert.eq(f("S"), "S")
  assert.eq(f("A"), "64943012")

  assert.eq(f("z zz zzz"), "PST PST PST")
  assert.eq(f("z", locale: "jp"), "GMT-8")
  assert.eq(f("zzzz"), "Pacific Standard Time")
  assert.eq(f("Z"), "-0800")
  assert.eq(f("ZZZZ"), "GMT-08:00")
  assert.eq(f("ZZZZZ"), "-08:00")
  assert.eq(f("O"), "GMT-8")
  assert.eq(f("OOOO"), "GMT-08:00")
  assert.eq(f("v"), "PT")
  assert.eq(f("vvvv"), "Pacific Time")
  assert.eq(f("V"), "uslax")
  // 'VV' not supported
  assert.eq(f("VVV"), "Los Angeles")
  assert.eq(f("VVVV"), "Los Angeles Time")
  assert.eq(f("X"), "-08")
  assert.eq(f("XX"), "-0800")
  assert.eq(f("XXX"), "-08:00")
  assert.eq(f("XXXX"), "-0800")
  assert.eq(f("XXXXX"), "-08:00")
  assert.eq(f("x"), "-08")
  assert.eq(f("xx"), "-0800")
  assert.eq(f("xxx"), "-08:00")
  assert.eq(f("xxxx"), "-0800")
  assert.eq(f("xxxxx"), "-08:00")
}
