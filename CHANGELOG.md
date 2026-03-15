# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

- Porting policy updated: parser groups will be aligned to the same regex definitions used in the Python fork, starting from upcoming METAR/TAF parser modules.

### Changed

- Removed `crates/metar-taf-cli`; the repository is now library-only with `metar-taf-core`.
- README updated to document the library-first direction and Python fork porting objective.

---

## [0.2.9] - 2026-03-15

### Added

- Added METAR runway-state regression tests for invalid runway designators and invalid data characters.

### Improved

- Hardened runway-state parsing to require numeric runway designators and digit-or-slash-only state payloads.

---

## [0.2.8] - 2026-03-15

### Added

- Added regression tests for trailing `=` terminator handling in both METAR and TAF inputs.

### Improved

- Tokenizer now strips trailing `=` from tokens, allowing last-group parsing to remain deterministic (e.g. `NOSIG=`, `FEW030=`).

---

## [0.2.7] - 2026-03-15

### Added

- Added METAR pressure parser regression tests for valid QNH/altimeter groups and malformed tokens.

### Improved

- Hardened METAR pressure parsing to accept only strict ICAO token formats (`Qdddd`, `Adddd`).
- Malformed pressure tokens are now ignored deterministically instead of being partially parsed.

---

## [0.2.6] - 2026-03-15

### Added

- Added TAF regression tests for invalid `TEMPO`/`BECMG` periods to ensure following tokens remain parseable.

### Improved

- Hardened `TEMPO` and `BECMG` parsing to consume period tokens only after successful period validation.
- Reused non-consuming lookahead strategy across change-group parsing helpers to avoid token swallowing on invalid groups.

---

## [0.2.5] - 2026-03-15

### Added

- Added METAR weather regression tests for `FZFG`, `VCBR`, `+SHGR`, unknown phenomena pairs, and malformed odd-length groups.

### Improved

- Hardened METAR weather parsing for descriptor/phenomena chaining and malformed trailing fragments.
- Unknown phenomena are now consumed in 2-character groups to preserve token progression deterministically.

---

## [0.2.4] - 2026-03-15

### Added

- Added TAF regression tests for `PROB40` without `TEMPO`, invalid `PROB` periods, and invalid `FM` times.

### Improved

- Hardened TAF change-group parsing to avoid consuming tokens when `PROBxx` period parsing fails.
- Added validation for `FMDDHHMM` and `DDHH/DDHH` period fragments before creating forecast groups.

---

## [0.2.3] - 2026-03-15

### Added

- TAF forecast parsing now supports statute-mile visibility in split-token form (e.g. `1 1/2SM`).
- Added dedicated TAF visibility statute-mile tests for both single-token and split-token forms.

### Improved

- Reused METAR split-token visibility parsing logic inside TAF forecast parsing to reduce behavioral drift.

---

## [0.2.2] - 2026-03-15

### Added

- METAR cloud parsing now supports `VV///` (vertical visibility with unknown height).
- Added cloud regression tests for `VV///` and invalid suffix handling (e.g. `SCT050ABC`).

### Improved

- Hardened cloud-layer parsing to reject unknown cloud suffixes and malformed layer lengths.

---

## [0.2.1] - 2026-03-15

### Added

- METAR wind parsing now supports explicit calm wind group `00000KT` with deterministic structured output.
- Added regression tests for calm wind and malformed wind groups (`36110KT`, `180ABKT`).

### Improved

- Hardened METAR wind parser validation for direction range and numeric speed/gust groups.

---

## [0.2.0-alpha5] - 2025-12-18

### Added

- Support for negative temperatures (Mxx/Mxx) in METAR
- Wind parsing with unit preservation (KT / MPS)
- Runway Visual Range (RVR) parsing (ICAO-compliant)
- Conservative handling of RMK (remarks) section
- Improved handling of CAVOK and cloud clearing
- Gust support for variable and fixed wind directions

### Fixed

- Correct handling of METAR/SPECI headers
- Fixed wind parsing regression with gusts
- Avoided misclassification of runway state groups as RVR

### Notes

- Runway state groups (e.g. R01/39//37) are currently ignored
- RMK is preserved as raw text without semantic parsing

---

## [0.2.0-alpha4] – 2025-12-18

### Added

- TAF parser now supports multiline inputs
- Support for `TAF AMD` and `TAF COR`
- Correct handling of `PROBxx` as forecast modifiers
- Tolerant parsing for unsupported TAF weather groups

### Improved

- CLI JSON output enriched with airport metadata

### Known limitations

- METAR advanced groups not yet supported:
    - `CAVOK`
    - wind speed in `MPS`
    - RVR groups (`Rxx/...`)
    - `RMK` / remarks

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
