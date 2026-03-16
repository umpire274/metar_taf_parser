# metar_taf_parser

> ⚠️ **Status:** Active development – current version `0.2.25`

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
metar-taf-parser = "0.2.25"
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
