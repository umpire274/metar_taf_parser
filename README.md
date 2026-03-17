# metar_taf_parser

> ⚠️ **Status:** Active development – current version `0.4.5`

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
metar-taf-parser = "0.4.5"
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

### Structured METAR remarks (RMK section)

The RMK section is parsed into typed variants instead of a raw string.
Access the raw text via `remarks.raw`; inspect parsed groups via `remarks.items`.

```rust
use metar_taf_parser::parse_metar;
use metar_taf_parser::metar::models::remark::{AutoStationKind, Remark};

let metar = parse_metar(
    "KORD 121750Z 18010KT 9999 FEW030 18/12 A2992 RMK AO2 SLP132 T02560178"
)?;

for remark in &metar.remarks.items {
    match remark {
        Remark::AutoStation(AutoStationKind::AO2) => {
            println!("automated station with precipitation discriminator");
        }
        Remark::SeaLevelPressure(hpa) => {
            println!("SLP: {:.1} hPa", hpa); // "SLP: 1013.2 hPa"
        }
        Remark::HourlyTemperature { temperature, dewpoint } => {
            println!("T: {:.1}°C  Td: {:.1}°C", temperature, dewpoint);
        }
        _ => {}
    }
}

// Raw text is always preserved:
assert_eq!(metar.remarks.raw, "AO2 SLP132 T02560178");

// Unrecognised tokens end up in remarks.unparsed:
let m2 = parse_metar("UOOO 181500Z 07002MPS CAVOK M25/M28 Q1014 RMK QFE746/0995")?;
assert_eq!(m2.remarks.unparsed, vec!["QFE746/0995"]);
```

Recognised remark variants: `PeakWind`, `WindShift`, `SeaLevelPressure`,
`PrecipitationAmount`, `HourlyTemperature`, `MaxMinTemperature`, `PressureTendency`,
`AutoStation`, `Lightning`, `Virga`, `MaintenanceIndicator`,
`PressureRisingRapidly`, `PressureFallingRapidly`, `SensorStatus`.

The `nosig` field on `Metar` is `true` whenever a `NOSIG` trend group is present.

---

### Wind direction variation (METAR)

When a variable direction range follows the wind group, it is stored in `wind.variation`:

```rust
use metar_taf_parser::parse_metar;

let m = parse_metar("LIRF 121250Z 18010KT 180V240 9999 FEW030 18/12 Q1015")?;
let v = m.wind.unwrap().variation.unwrap();
assert_eq!(v.min, 180);
assert_eq!(v.max, 240);
```

### Icing and turbulence in TAF forecasts

Icing (`6ABBBC`) and turbulence (`5ABBBC`) groups are parsed into typed vectors on each
`TafForecast` block:

```rust
use metar_taf_parser::parse_taf;
use metar_taf_parser::taf::models::icing::IcingIntensity;
use metar_taf_parser::taf::models::turbulence::TurbulenceIntensity;

let taf = parse_taf(
    "TAF KORD 121100Z 1212/1318 18010KT 9999 SCT020 610304 520803"
)?;

let fc = &taf.forecasts[0];

// Icing: 6 + intensity=1(Light) + base=030(3000 ft) + thickness=4(4000 ft)
assert_eq!(fc.icing[0].intensity, IcingIntensity::Light);
assert_eq!(fc.icing[0].base_ft, 3000);

// Turbulence: 5 + intensity=2(ModerateInCloud) + base=080(8000 ft) + thickness=3(3000 ft)
assert_eq!(fc.turbulence[0].intensity, TurbulenceIntensity::ModerateInCloud);
assert_eq!(fc.turbulence[0].base_ft, 8000);
```

---

### SPECI (special observation)

The parser recognises `SPECI` as an optional leading token alongside `METAR`.
The report type is stored in `metar.report_type` and reflected in `MetarDescription::report_type`.

```rust
use metar_taf_parser::parse_metar;
use metar_taf_parser::metar::models::report_type::MetarReportType;

let m = parse_metar("SPECI EGLL 121250Z 24015KT 0800 FG VV002 08/07 Q1008")?;
assert_eq!(m.report_type, MetarReportType::Speci);
```

---

### Military color code (NATO/UK MIL-METAR)

Military airfield METARs carry a NATO color state (`BLU`, `WHT`, `GRN`, `YLO`, `AMB`, `RED`).
A `BLACK` variant (`BLU+`, etc.) indicates the field is closed for that state.
A second bare color code token is treated as the implicit BECMG forecast state.

```rust
use metar_taf_parser::parse_metar;
use metar_taf_parser::metar::models::color_code::{MilitaryColor, MilitaryColorCode};

let m = parse_metar("METAR EGVN 120930Z 25010KT 7000 BKN020 15/10 Q1013 GRN WHT")?;
assert_eq!(m.color_code.as_ref().map(|c| &c.code), Some(&MilitaryColorCode::Grn));
assert_eq!(m.color_code_forecast.as_ref().map(|c| &c.code), Some(&MilitaryColorCode::Wht));
```

---

### Sea state (offshore stations)

Offshore and ship stations report water temperature and wave state using `W<TT>/S<n>` or
`W<TT>/H<hh>` tokens. The optional `M` prefix encodes negative temperatures.

```rust
use metar_taf_parser::parse_metar;
use metar_taf_parser::metar::models::sea_state::WaveHeightKind;

let m = parse_metar("METAR EKCH 120930Z 25010KT 9999 FEW020 12/08 Q1013 W12/S8")?;
let ss = m.sea_state.unwrap();
assert_eq!(ss.water_temperature, Some(12));
assert_eq!(ss.wave_kind, WaveHeightKind::StateCode);
assert_eq!(ss.wave_value, Some(8));
```

---

### Wind shear on runways (METAR)

The `WS R<rwy>` and `WS ALL RWY` groups are parsed into a typed `Vec` on the `Metar` struct.

```rust
use metar_taf_parser::parse_metar;
use metar_taf_parser::metar::models::wind_shear::MetarWindShearRunway;

let m = parse_metar("METAR EGLL 120930Z 25010KT 9999 FEW020 15/10 Q1013 WS ALL RWY")?;
assert_eq!(m.wind_shear[0], MetarWindShearRunway::AllRunways);

let m2 = parse_metar("METAR EGLL 120930Z 25010KT 9999 FEW020 15/10 Q1013 WS R23")?;
assert_eq!(m2.wind_shear[0], MetarWindShearRunway::Runway("23".to_string()));
```

---

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
