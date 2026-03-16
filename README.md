# metar_taf_parser

> ⚠️ **Status:** Active development – current version `0.2.19`

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
metar-taf-parser = "0.2.19"
```

Example:

```rust
use metar_taf_parser::parse_metar;

let metar = parse_metar(
    "LIRF 121250Z 18012KT 9999 FEW030 SCT080 18/12 Q1015"
)?;

println!("{:#?}", metar);
```

For strict parsing (error on unsupported groups), use `parse_taf_strict`.

---

## 🧪 Testing

```bash
cargo test
```

---

## 📄 License

[MIT](LICENSE) License.
