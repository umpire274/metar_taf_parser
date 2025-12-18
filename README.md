# metar_taf_parser

> ⚠️ **Status:** Active development – current version `0.2.0-alpha4`
>
> Note: METAR parsing is intentionally partial in this alpha release. Advanced groups (CAVOK, RVR, MPS, RMK) will be
> added in a future alpha.

A modern, strongly-typed **METAR and TAF parser** written in Rust.

`metar_taf_parser` provides a reusable parsing library and a command-line interface
to parse aviation weather reports (METAR and TAF) into structured data,
with full support for advanced weather phenomena.

---

## ✈️ Features

### METAR parsing

- Station identifier
- Observation time (`ddhhmmZ`)
- Wind:
    - direction, speed, gusts
    - variable wind
- Visibility:
    - prevailing visibility
    - minimum visibility with direction (e.g. `2000SW`)
- Cloud layers:
    - FEW, SCT, BKN, OVC
    - cloud height
    - cloud types (CB, TCU)
- Temperature and dew point
- Pressure (QNH)
- **Advanced weather phenomena**:
    - intensity (`-`, `+`)
    - descriptors (`TS`, `SH`, `FZ`, `VC`, …)
    - phenomena (`RA`, `SN`, `BR`, `FG`, …)
    - multiple simultaneous weather groups

### TAF parsing

- Full TAF header parsing (station, issue time, validity)
- Forecast groups:
    - Base forecast
    - `FM`
    - `BECMG`
    - `TEMPO`
    - `PROB30` / `PROB40` (with or without `TEMPO`)
- Reuse of METAR parsing logic for:
    - wind
    - visibility
    - clouds
    - weather
- Explicit forecast period modeling

---

## 🧱 Architecture

The project is structured as a Rust workspace:

```text
metar_taf_parser/
├── crates/
│ ├── metar-taf-core/ # Parsing library
│ └── metar-taf-cli/ # Command-line interface
├── Cargo.toml
├── README.md
└── CHANGELOG.md
```

### Core library (`metar-taf-core`)

- Token-based parsing
- Strongly typed domain models
- Designed to be embedded in other applications
- No I/O, no CLI assumptions

### CLI (`metar-taf-cli`)

- Simple interface for parsing METAR / TAF strings
- Intended mainly for inspection and testing
- Will evolve in future releases

---

## 🚀 Installation

### From source

```bash
git clone https://github.com/<your-org-or-user>/metar_taf_parser.git
cd metar_taf_parser
cargo build --release
```

---

## 🖥️ CLI usage

### Fetch METAR / TAF reports

```bash
metar-taf <ICAO> get --metar
metar-taf <ICAO> get --taf
metar-taf <ICAO> get --all
```

If the ICAO code is not provided, the CLI will prompt interactively:

```bash
metar-taf get --metar
Enter ICAO airport code:
```

The CLI fetches the latest available data from the NOAA Aviation Weather service and automatically parses the returned
METAR or TAF.

### Output modes

By default, the CLI outputs the parsed data in debug format:

```bash
# Parsed output (default)
metar-taf LIRF get --metar

# Raw METAR / TAF only
metar-taf LIRF get --metar --raw

# JSON output
metar-taf LIRF get --metar --json
```

**Notes**:

- `--raw` outputs only the raw report string
- `--json` outputs only JSON
- `--raw` and `--json` cannot be used together

---

## 📚 Library usage

Add the core crate to your Cargo.toml:

```toml
[dependencies]
metar-taf-core = "0.1.0"
```

Example:

```rust
use metar_taf_core::parse_metar;

let metar = parse_metar(
"LIRF 121250Z 18012KT 9999 FEW030 SCT080 18/12 Q1015"
) ?;

println!("{:#?}", metar);
```

---

## 🧪 Testing & quality

- Unit tests for individual parsers
- Golden tests using real-world METAR and TAF reports
- `cargo clippy` clean with `-D warnings`

---

## 📦 Versioning

This project follows **Semantic Versioning**.

- `0.1.0` is the **initial stable release**
- Public API is considered **experimental** and may evolve in future minor versions

See [CHANGELOG.md](CHANGELOG.md) for details.

---

## 🛣️ Roadmap

Planned for upcoming `0.2.x` releases:

- ICAO validation and airport metadata lookup
- Golden JSON snapshot tests
- Human-readable pretty output
- Extended TAF support (INTER, CNL, AMD)

---

### 📄 License

[MIT](LICENSE) License.

---

### ✍️ Author

Developed and maintained by **Alessandro Maestri**.
