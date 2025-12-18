# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0-alpha3] – 2025-12-18

### Added

#### ICAO validation

- Real ICAO airport code validation using an embedded airport database
- Airports database loaded from `airports.dat` in the core library
- Early error reporting for unknown ICAO codes

#### CLI behavior

- ICAO positional argument is required when using `--json`
- Interactive prompt disabled automatically in non-interactive / JSON mode

### Notes

- This is an **alpha release**.
- Airport metadata is currently used for validation only.

---

## [0.2.0-alpha2] – 2025-12-18

### Added

#### CLI output

- New `--json` flag to output parsed METAR and TAF data in JSON format
- New `--raw` flag to output raw METAR or TAF strings only
- `--json` and `--raw` are mutually exclusive (validated by clap)

#### CLI behavior

- Cleaner and more predictable output modes:
    - default: parsed debug output
    - `--json`: machine-readable JSON only
    - `--raw`: raw report only
- Improved scriptability and pipeline integration

### Notes

- This is an **alpha release**.
- Output formats are stabilizing but may still change in future alpha versions.

---

## [0.2.0-alpha1] – 2025-12-18

### Added

#### CLI

- New `get` command to fetch METAR and TAF reports by ICAO airport code
- Support for positional ICAO argument or interactive prompt if omitted
- Mutually exclusive flags:
    - `--metar`
    - `--taf`
    - `--all`

#### Data fetching

- Integration with NOAA Aviation Weather API
- Real-time retrieval of raw METAR and TAF strings

#### Parsing integration

- Automatic parsing of fetched METAR and TAF reports using the core library
- CLI output includes both raw and parsed representations (debug format)

### Notes

- This is an **alpha release**.
- CLI output format is not stable and will change in future alpha versions.

---

## [0.1.0] – 2025-12-18

### Added

#### METAR parsing

- Full METAR string parsing with token-based architecture
- Support for:
    - Station identifier
    - Observation time (ddhhmmZ)
    - Wind (direction, speed, gusts, variable wind)
    - Visibility, including:
        - prevailing visibility
        - minimum visibility with direction (e.g. `2000SW`)
    - Cloud layers (FEW, SCT, BKN, OVC) with height and cloud type (CB, TCU)
    - Temperature and dew point
    - QNH / pressure
- Advanced weather phenomena parsing:
    - Intensity (`-`, `+`)
    - Descriptors (e.g. `TS`, `SH`, `FZ`, `VC`)
    - Phenomena (e.g. `RA`, `SN`, `BR`, `FG`)
    - Support for multiple simultaneous weather groups

#### TAF parsing

- Full TAF header parsing (station, issue time, validity period)
- Forecast groups support:
    - Base forecast
    - FM (From)
    - BECMG
    - TEMPO
    - PROB30 / PROB40 (with or without TEMPO)
- Reuse of METAR parsing logic for wind, visibility, clouds, and weather
- Structured forecast model with explicit period handling

#### Core architecture

- Clean separation between:
    - parsing logic
    - domain models
    - shared utilities
- Strongly typed models for METAR, TAF, forecasts, weather, clouds, and visibility
- Designed as a reusable parsing library (`metar-taf-core`)

#### CLI

- Initial command-line interface (`metar-taf-cli`)
- Parse and inspect METAR / TAF strings from standard input

#### Testing & quality

- Golden tests based on real-world METAR and TAF examples
- Unit tests for individual parsing components
- `cargo clippy` clean with `-D warnings`

---

### Notes

- This is the **initial stable release** of the project.
- Public API is considered **experimental** and may evolve in future minor versions.

[0.1.0]: https://github.com/<your-org-or-user>/metar_taf_parser/releases/tag/v0.1.0

[0.2.0-alpha1]: https://github.com/<your-org-or-user>/metar_taf_parser/releases/tag/v0.2.0-alpha1

[0.2.0-alpha2]: https://github.com/<your-org-or-user>/metar_taf_parser/releases/tag/v0.2.0-alpha2
