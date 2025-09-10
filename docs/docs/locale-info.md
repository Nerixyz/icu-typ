# `locale-info`

```typst-code
let locale-info(locale)
```

Gets information about ICU4X' understanding of the `locale`.

## Arguments

### `locale`

The [Unicode Locale Identifier] to parse.

## Structure

A [Unicode Locale Identifier] starts with a [Unicode **Language** Identifier](https://unicode.org/reports/tr35/tr35.html#unicode-language-identifier) and is optionally followed by extensions. Most of the time the language identifier is sufficient to get the desired output.

The language identifier starts with a [language subtag] with an optional [script subtag] _or_ just a [script subtag]. Afterwards, the [region subtag] and [variant subtag] can be specified.

In this library, only some of the [Unicode extensions](https://unicode.org/reports/tr35/tr35.html#Key_And_Type_Definitions_) (`-u-` extensions) are important.

- `ca` can be used to specify the calendar for dates and datetimes ([calendar.xml](https://github.com/unicode-org/cldr/blob/main/common/bcp47/calendar.xml))
- `hc` can be used to specify the hour cycle for time formatters ([reference](https://unicode.org/reports/tr35/tr35.html#UnicodeHourCycleIdentifier))

### Example

The locale `zh-Hans-CN` specifies the [language subtag] as `zn` (Chinese), the [script subtag] as `Hans` (simplified Han), and the [region subtag] as `CN` (China).

You can test your identifiers on [util.unicode.org/UnicodeJsps/languageid.jsp](https://util.unicode.org/UnicodeJsps/languageid.jsp?l=en).

## Examples

### Language Identifier

Chinese, tranditional Han, Hong Kong

```typst +preview
#icu.locale-info("zh-Hant-HK")
```

### Default Language

The default "undefined" language.

```typst +preview
#icu.locale-info("und")
```

### Full Unicode Locale Identifier

A [Unicode Locale Identifier] specifying all extensions.

```typst +preview(vertical)
#let language-id = "en-arab-DE-posix-macos"
#let unicode-ext = "-u-foo-bar-hc-h12-ca-buddhist"
#let transform-ext = "-t-en-latn-US-windows-rusty-h0-hybrid"
#let private-ext = "-a-other-ext"
#let other-ext = "-x-typst-wasm"

#icu.locale-info(
    language-id +
    unicode-ext +
    transform-ext +
    private-ext +
    other-ext
)
```

[Unicode Locale Identifier]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
[language subtag]: https://unicode.org/reports/tr35/tr35.html#unicode_language_subtag_validity
[script subtag]: https://unicode.org/reports/tr35/tr35.html#unicode_script_subtag_validity
[region subtag]: https://unicode.org/reports/tr35/tr35.html#unicode_region_subtag_validity
[variant subtag]: https://unicode.org/reports/tr35/tr35.html#unicode_variant_subtag_validity
