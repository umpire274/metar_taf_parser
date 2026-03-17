# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.4.0] - 2026-03-17

### Added

- Added `common::describe` module providing natural language output for parsed METAR and TAF reports.
- Added `describe_metar(metar: &Metar, lang: Language) -> MetarDescription` public function.
- Added `describe_taf(taf: &Taf, lang: Language) -> TafDescription` public function.
- Added `Language` enum (`En` variant) to select the output language; defaults to English.
- Added `MetarDescription` struct with human-readable `Option<String>` fields for each METAR group
  (`station`, `time`, `modifier`, `wind`, `visibility`, `weather`, `clouds`, `temperature`, `pressure`, `trend`, `remarks`).
- Added `TafDescription` and `ForecastDescription` structs for TAF natural language output.
- Added `Locale` trait (`common::describe::locale`) to allow future language implementations
  without touching parser or domain model code.
- Added English locale (`locale::en::En`) covering all current METAR and TAF domain types.
- Re-exported `describe_metar`, `describe_taf`, `Language`, `MetarDescription`, `TafDescription`,
  and `ForecastDescription` from `lib.rs` for direct crate-level access.
- Added 33 integration tests across `tests/describe_metar.rs` and `tests/describe_taf.rs` covering
  wind (directional, variable, gust), visibility (CAVOK, metres, >10 km), cloud layers (with CB/TCU),
  weather phenomena (intensity, descriptor, phenomenon), temperature (positive and negative),
  pressure (QNH and inHg), modifiers (AUTO, COR, AMD), trends (NOSIG, TEMPO+time),
  remarks preservation, TAF blocks (Base, FM, BECMG, TEMPO, PROB), wind shear, and temperatures.

### Changed

- Bumped crate version to `0.4.0`.
- README updated with `describe_metar` and `describe_taf` usage examples and dependency snippet.

---

## [0.3.0] - 2026-03-16

### Added

- Declared porting-baseline completion for METAR/TAF parser parity and moved roadmap guidance to maintenance mode.

### Changed

- Bumped crate version and README dependency markers to `0.3.0`.
- Updated porting status notes in `docs/PORTING_REGEX_POLICY.md` and `docs/CODEBASE_REFERENCE.md` to reflect baseline completion.

---

## [0.2.25] - 2026-03-16

### Added

- Added `parse_metar_strict`, which rejects METAR messages containing unsupported/unparsed groups (including trend-detail unknown payload tokens).
- Added strict-mode METAR regression tests and README usage example.

### Changed

- Completed strict/tolerant review milestone for both METAR and TAF parser flows.
- Bumped crate version and README dependency markers to `0.2.25`.

---

## [0.2.24] - 2026-03-16

### Added

- Added TAF weather payload coverage for `NSW` tokens across change groups (`BECMG`, `TEMPO`, `PROBxx`) with regression tests.

### Changed

- Bumped crate version and README dependency markers to `0.2.24`.

---

## [0.2.23] - 2026-03-16

### Changed

- Synced porting policy immediate-next modules with the current parser status after completing RVR, `TX`/`TN`, and `WS` work.
- Corrected historical changelog references for crate naming and README dependency snippets after the root-crate migration.
- Updated README and manifest version markers to `0.2.23`.

---

## [0.2.22] - 2026-03-16

### Added

- Added TAF forecast wind-shear (`WS`) support with structured fields for shear height, direction, and speed.
- Added regression tests for valid/malformed wind-shear groups and strict-mode rejection behavior.

### Improved

- TAF forecast parsing now captures `WSddd/dddffKT` tokens without interfering with existing group parsing flow.

---

## [0.2.21] - 2026-03-16

### Added

- Added TAF forecast temperature support for `TX`/`TN` groups with structured model fields (`value`, `day`, `hour`).
- Added regression tests for valid positive/negative TAF temperature groups and malformed temperature tokens.

### Improved

- TAF forecast parsing now captures `TX`/`TN` tokens without affecting existing change-group and visibility/weather parsing flow.

---

## [0.2.20] - 2026-03-16

### Added

- Added dedicated METAR RVR (runway visual range) parsing with structured model fields (qualifier, variable range, tendency, and unit).
- Added regression tests covering RVR valid/invalid tokens and coexistence with runway-state groups.

### Improved

- METAR parsing now captures RVR groups separately from runway-state groups, avoiding cross-group misclassification.

---

## [0.2.19] - 2026-03-16

### Changed

- Promoted the core library crate to repository root and renamed package to `metar-taf-parser` for crates.io publication.
- Removed workspace-style root manifest and replaced it with the single-package crate manifest.
- Updated README usage examples and test commands to the new root crate name/layout.

### Notes

- Existing Rust import path is now `metar_taf_parser` (legacy import path was `metar_taf_core`).

---

## [0.2.18] - 2026-03-16

### Added

- Added `parse_taf_strict`, which rejects TAF messages containing unsupported/unparsed groups.
- Added regression tests for tolerant vs strict TAF parsing behavior.

### Changed

- Updated README examples to reference `metar-taf-parser = "0.2.18"` and document strict TAF parsing mode.

---

## [0.2.17] - 2026-03-16

### Changed

- Updated `thiserror` dependency to `2.0.18`.
- README dependency snippet now matches current crate line (`metar-taf-parser = "0.2.17"`).

### Improved

- Extended METAR trend parsing to recognize `BECMG` and `TEMPO` marker tokens in addition to `NOSIG`.
- Updated porting policy "Immediate next modules" to reflect remaining parity targets (RVR, advanced METAR trend payloads, TAF `TX`/`TN` + wind-shear sections).

### Added

- Added regression tests for `NOSIG`, `BECMG`, and `TEMPO` METAR trend markers.

---

## [0.2.16] - 2026-03-16

### Added

- Added diagnostics regression tests for unknown/unparsed groups in both METAR and TAF parsing flows.

### Improved

- METAR parser now stores unknown tokens in `unparsed_groups` instead of silently dropping them.
- TAF parser now exposes `unparsed_groups` and collects tokens that are not consumed by forecast group parsers.

---

## [0.2.15] - 2026-03-16

### Added

- Added TAF regression checks that assert weather parsing across base, `TEMPO`, and `PROBxx` forecast groups.

### Improved

- Extended TAF forecast model/parser to capture weather groups (e.g. `-RA`, `TSRA`, `SHRA`) in structured output.

---

## [0.2.14] - 2026-03-16

### Improved

- Tightened TAF validity hour checks so the start hour (`from`) is limited to `00..23`, while preserving `24` support only for validity end hour (`to`).

### Added

- Added regression coverage for invalid `from` hour `24` in TAF validity groups.

---

## [0.2.13] - 2026-03-16

### Added

- Added TAF modifier regression tests for `COR` parsing (with and without the `TAF` prefix).

### Improved

- Extended TAF header parsing to recognize `COR` as `ReportModifier::Correction` in addition to existing `AMD`/`NIL` handling.

---

## [0.2.12] - 2026-03-16

### Added

- Added TAF time/validity regression tests for malformed widths, non-numeric values, and out-of-range components.

### Improved

- Hardened TAF issue-time parsing to require strict `DDHHMMZ` numeric tokens and valid day/hour/minute ranges.
- Hardened TAF validity parsing to require strict `DDHH/DDHH` numeric tokens and valid day/hour ranges.

---

## [0.2.11] - 2026-03-15

### Added

- Added METAR temperature regression tests for malformed token width and out-of-range values.

### Improved

- Hardened METAR temperature parsing to require 2-digit components (with optional `M` prefix) and conservative range validation.

---

## [0.2.10] - 2026-03-15

### Added

- Added METAR time regression tests for valid groups plus invalid range/non-numeric cases.

### Improved

- Hardened METAR time parsing to validate `DDHHMMZ` numeric content and range limits (day/hour/minute).

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

[0.1.0]: releases/tag/v0.1.0

[0.2.0-alpha1]: releases/tag/v0.2.0-alpha1

[0.2.0-alpha2]: releases/tag/v0.2.0-alpha2
