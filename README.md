# metar_taf_parser

> ⚠️ **Status:** Active development – current version `0.4.0`

A modern, strongly-typed **METAR and TAF parser library** written in Rust.

This project is now focused on the single library crate (`metar-taf-parser`) and is a
Rust port of the original Python project:
https://github.com/mivek/python-metar-taf-parser

Porting and parity work also tracks the maintained fork:
https://github.com/umpire274/python-metar-taf-parser.

---

## ✈️ Goals

- Provide a robust Rust-native API for METAR/TAF parsing.
- Reach feature parity with the referenced Python fork.
- Keep the public API ergonomic and strongly typed for embedding in other apps.
- During porting, parser groups are being migrated using the same regex-first approach used in the Python fork to preserve behavior.
- Porting process follows a regex-alignment policy documented in [`docs/PORTING_REGEX_POLICY.md`](docs/PORTING_REGEX_POLICY.md).

---

## 🧱 Architecture

The repository now exposes a single library crate at the root:

```text
metar_taf_parser/
├── src/
├── tests/
├── resources/
├── Cargo.toml
├── README.md
└── CHANGELOG.md
```

### Core library (`metar-taf-parser`)

- Token-based parsing
- Strongly typed domain models
- Designed to be embedded in other applications
- No CLI assumptions

---

## 📚 Library usage

Add the core crate to your `Cargo.toml`:

```toml
[dependencies]
metar-taf-parser = "0.4.0"
```

### METAR example

```rust
use metar_taf_parser::parse_metar;

let metar = parse_metar(
    "LIRF 121250Z 18012KT 9999 FEW030 SCT080 18/12 Q1015"
)?;

assert_eq!(metar.station, "LIRF");
assert!(metar.wind.is_some());
assert!(metar.temperature.is_some());
```

### TAF example (tolerant mode)

```rust
use metar_taf_parser::parse_taf;

let taf = parse_taf(
    "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 TX18/1214Z TN08/1304Z TEMPO 1218/1222 4000 -RA"
)?;

assert_eq!(taf.station, "LIRF");
assert!(!taf.forecasts.is_empty());
// In tolerant mode, unsupported tokens are preserved in `unparsed_groups`.
```

### METAR strict mode example

```rust
use metar_taf_parser::parse_metar_strict;

let strict_result = parse_metar_strict(
    "LIRF 121250Z 18010KT 9999 FEW030 UNKNOWN 18/12 Q1015"
);

assert!(strict_result.is_err());
```

### TAF strict mode example

```rust
use metar_taf_parser::parse_taf_strict;

let strict_result = parse_taf_strict(
    "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 UNKNOWN_TOKEN"
);

assert!(strict_result.is_err());
```

### Natural language description — field access (METAR)

Use `describe_metar` when you need to inspect individual fields programmatically.

```rust
use metar_taf_parser::{parse_metar, describe_metar, Language};

let metar = parse_metar(
    "LIRF 121250Z 18010KT 9999 FEW030 18/12 Q1015"
)?;

let desc = describe_metar(&metar, Language::En);

println!("{}", desc.wind.unwrap());
// "wind from 180° at 10 kt"

println!("{}", desc.visibility.unwrap());
// "visibility greater than 10 km"

println!("{}", desc.clouds[0]);
// "few clouds at 3000 ft"

println!("{}", desc.temperature.unwrap());
// "temperature 18°C, dew point 12°C"

println!("{}", desc.pressure.unwrap());
// "QNH 1015 hPa"
```

### Natural language description — full formatted output (METAR)

Use `format_metar` (or `println!("{}", desc)`) to print the complete report in one call.
Only the fields actually present in the message are included.

```rust
use metar_taf_parser::{parse_metar, format_metar, Language};

let metar = parse_metar(
    "LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG"
)?;

println!("{}", format_metar(&metar, Language::En));
```

```text
METAR LIRF
  Time:        Day 12 at 12:50Z
  Wind:        wind from 180° at 10 kt
  Visibility:  visibility greater than 10 km
  Weather:     light rain
  Clouds:      few clouds at 3000 ft
  Temperature: temperature 18°C, dew point 12°C
  Pressure:    QNH 1015 hPa
  Trend:       no significant change
```

### Natural language description — field access (TAF)

```rust
use metar_taf_parser::{parse_taf, describe_taf, Language};

let taf = parse_taf(
    "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 TX18/1214Z TN08/1304Z TEMPO 1218/1222 4000 -RA"
)?;

let desc = describe_taf(&taf, Language::En);

println!("{}", desc.validity.unwrap());
// "12/12Z to 13/18Z"

for forecast in &desc.forecasts {
    println!("[{}]", forecast.kind);
    if let Some(wind) = &forecast.wind { println!("  {}", wind); }
    if let Some(vis)  = &forecast.visibility { println!("  {}", vis); }
    for cloud in &forecast.clouds { println!("  {}", cloud); }
}
// [Base forecast]
//   wind from 180° at 10 kt
//   visibility greater than 10 km
//   scattered clouds at 2000 ft
// [Temporary]
//   visibility 4000 m
//   light rain
```

### Natural language description — full formatted output (TAF)

```rust
use metar_taf_parser::{parse_taf, format_taf, Language};

let taf = parse_taf(
    "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020 TX18/1214Z TN08/1304Z TEMPO 1218/1222 4000 -RA"
)?;

println!("{}", format_taf(&taf, Language::En));
```

```text
TAF LIRF
  Issued:    Day 12 at 11:00Z
  Validity:  12/12Z to 13/18Z
  [Base forecast]
    Wind:        wind from 180° at 10 kt
    Visibility:  visibility greater than 10 km
    Clouds:      scattered clouds at 2000 ft
    Max temp:    maximum temperature 18°C on day 12 at 14:00Z
    Min temp:    minimum temperature 8°C on day 13 at 04:00Z
  [Temporary]  12/18Z to 12/22Z
    Visibility:  visibility 4000 m
    Weather:     light rain
```

### Typical parser use cases

- Parse a single METAR and inspect typed fields (`wind`, `visibility`, `clouds`, `temperature`, `pressure`).
- Parse a TAF and iterate forecast sections (`BASE`, `FM`, `BECMG`, `TEMPO`, `PROB`).
- Enable strict TAF mode in validation pipelines where unknown groups must fail fast.
- Use tolerant mode when you want best-effort parsing plus visibility on unsupported tokens.

---

## 📘 Documentation

- Porting policy: [`docs/PORTING_REGEX_POLICY.md`](docs/PORTING_REGEX_POLICY.md)
- Full codebase reference (modules, structs, enums, functions): [`docs/CODEBASE_REFERENCE.md`](docs/CODEBASE_REFERENCE.md)

---

## 🧪 Testing

```bash
cargo test
```

---

## 📄 License

[MIT](LICENSE) License.
