# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.4.7] - 2026-03-17

### Fixed

- **TAF parser — Gruppo 0 (codice TAF):** `TAF AMD NIL` e `TAF COR NIL` non venivano
  riconosciuti come report NIL. Il token `NIL` era erroneamente trattato come stazione,
  producendo un `Taf` con `station = "NIL"` invece di `modifier = Nil`. Il parser ora
  verifica se il token successivo a `AMD` o `COR` è `NIL` e restituisce il report NIL
  correttamente, allineandosi al comportamento già presente nel parser METAR per `COR NIL`.

### Added

- Esteso `tests/taf_modifier.rs` con 7 nuovi test che coprono l'intero Gruppo 0:
  - `parse_taf_without_prefix_normal` — TAF senza token `TAF` iniziale.
  - `parse_taf_amd_nil` — regressione per il bug `TAF AMD NIL`.
  - `parse_taf_cor_nil` — regressione per il bug `TAF COR NIL`.
  - `describe_taf_modifier_amd` — `modifier` → `"amended forecast"`.
  - `describe_taf_modifier_cor` — `modifier` → `"corrected report"`.
  - `describe_taf_modifier_nil` — `modifier` → `"no data available"`.
  - `describe_taf_modifier_normal_is_absent` — `modifier` è `None` per report normali.

- Aggiunto `tests/taf_change_groups.rs` con 23 test per i gruppi evolutivi del TAF:
  - **FM (From):** blocco autonomo, campi corretti, minuti non zero, multipli, periodo
    invalido ignorato.
  - **BECMG (Becoming):** esempio dal manuale (`BECMG 2521/2523 22013KT`), periodo ≤ 4 ore,
    elementi non modificati assenti, attraversamento mezzanotte, multipli.
  - **TEMPO (Temporary):** esempio dal manuale (`TEMPO 2600/2603 TSRA`), periodo corretto,
    attraversamento mezzanotte, multipli, TEMPO dopo FM.
  - **Combinazioni:** tutti i gruppi evolutivi nello stesso TAF, indipendenza dei contenuti.
  - **Describe:** label FM/BECMG/TEMPO, periodo BECMG, periodo TEMPO.

- Esteso `tests/taf_prob.rs` da 4 a 14 test per i gruppi probabilità (`PROB30`/`PROB40`):
  - Esempio dal manuale: `PROB40 2510/2513` con verifica di periodo, kind e probabilità.
  - Validazione: `PROB50` non deve creare un blocco PROB (solo 30 e 40 sono ammessi).
  - `PROB30` e `PROB40` riconosciuti con il corretto valore `probability`.
  - Attraversamento mezzanotte con `PROB30`.
  - Combinazione PROB con FM e TEMPO nello stesso TAF.
  - Elementi non modificati assenti nel blocco PROB.
  - Describe: label "Probability", campo `probability` con percentuale, campo `period`.

- Esteso `tests/taf_temperature_groups.rs` da 3 a 16 test per i gruppi temperatura
  (`TX`/`TN`):
  - Esempi dal manuale: `TX22/1718Z` (max 22 °C alle 18:00Z del 17°) e `TNM01/1801Z`
    (min −1 °C alle 01:00Z del 18°).
  - Combinazione TX + TN nello stesso TAF.
  - Solo TX senza TN e solo TN senza TX.
  - Temperatura zero (`TX00`/`TN00`).
  - TX/TN con giorno che attraversa la mezzanotte.
  - TX all'interno di un blocco FM.
  - Token malformati: mancanza del suffisso `Z`, parte oraria non numerica.
  - Describe: `max_temperature` contiene valore/giorno/ora; `min_temperature` contiene
    valore negativo; formato con `"maximum"` e `"°C"`.

### Changed

- Bumped crate version to `0.4.7`.
- README and CHANGELOG updated.

---

## [0.4.6] - 2026-03-17

### Added

- Added `common::parse` module providing a unified entry-point for METAR and TAF parsing.
- Added `ParsedReport` enum:
  - `ParsedReport::Metar(Metar)` — wraps a successfully decoded METAR or SPECI.
  - `ParsedReport::Taf(Taf)` — wraps a successfully decoded TAF.
- Added `ParseError` enum (derives `thiserror::Error`):
  - `ParseError::Metar(MetarError)` — inner METAR parser failure.
  - `ParseError::Taf(TafError)` — inner TAF parser failure.
  - `ParseError::UnknownReportType` — leading token is neither `METAR`, `SPECI`, nor `TAF`
    (only returned by `parse_strict`).
- Added `parse(input: &str) -> Result<ParsedReport, ParseError>` — tolerant dispatcher:
  - leading token `TAF` → delegates to `parse_taf`.
  - any other prefix (including `METAR`, `SPECI`, or none) → delegates to `parse_metar`.
- Added `parse_strict(input: &str) -> Result<ParsedReport, ParseError>` — strict dispatcher:
  - requires an explicit `METAR`, `SPECI`, or `TAF` leading token; returns
    `ParseError::UnknownReportType` otherwise.
  - delegates to `parse_metar_strict` / `parse_taf_strict` respectively.
- Re-exported `parse`, `parse_strict`, `ParsedReport`, and `ParseError` from `lib.rs`.
- Added 13 integration tests in `tests/parse_unified.rs` covering tolerant and strict
  dispatch for all three prefixes, the no-prefix fallback, station extraction from both
  variants, tolerant unknown-group collection, and strict rejection of unknown groups.

### Changed

- Bumped crate version to `0.4.6`.
- README and CHANGELOG updated.

---

## [0.4.5] - 2026-03-17

### Added

- Added `metar::models::color_code` module with:
  - `MilitaryColorCode` enum (`Blu`, `Wht`, `Grn`, `Ylo`, `Amb`, `Red`).
  - `MilitaryColor { code, black }` struct — `black: true` for `BLU+` / `WHT+` variants.
- Added `color_code: Option<MilitaryColor>` and `color_code_forecast: Option<MilitaryColor>`
  fields to `Metar`; the first bare color code token sets `color_code`, a second bare token
  (without `BECMG` keyword) sets `color_code_forecast`.
- Added `describe_military_color` helper in `common::describe::fields`.
- Added `color_code` and `color_code_forecast` fields to `MetarDescription`; printed by
  `Display` / `format_metar`.
- Added 15 integration tests in `tests/metar_color_code.rs` covering all six codes, `BLACK`
  variants, implicit BECMG forecast code, and describe output.

- Added `metar::models::sea_state` module with:
  - `WaveHeightKind` enum (`StateCode` / `Height`).
  - `SeaState { water_temperature, wave_kind, wave_value }` struct.
- Added `sea_state: Option<SeaState>` field to `Metar`.
- Added `describe_sea_state` helper in `common::describe::fields`.
- Added `sea_state: Option<String>` field to `MetarDescription`.
- Added 10 integration tests in `tests/metar_sea_state.rs`.

- Added `metar::models::wind_shear` module with:
  - `MetarWindShearRunway` enum (`Runway(String)` / `AllRunways`).
- Added `wind_shear: Vec<MetarWindShearRunway>` field to `Metar`; parser recognises
  `WS R<rwy>` (specific runway) and `WS ALL RWY` (all runways).
- Added `describe_metar_wind_shear_runway` helper in `common::describe::fields`.
- Added `wind_shear: Vec<String>` field to `MetarDescription`.
- Added 8 integration tests in `tests/metar_wind_shear_runway.rs`.

- Added `metar::models::report_type` module with `MetarReportType { Metar, Speci }` enum.
- Added `report_type: MetarReportType` field to `Metar`; defaults to `Metar`.
- Parser now recognises `SPECI` as an optional leading token (in addition to `METAR`),
  setting `report_type` to `Speci` accordingly. Report type is preserved even in NIL
  and COR NIL early-return paths.
- Added `report_type: String` field to `MetarDescription` (always `"METAR"` or `"SPECI"`).
- Added 8 tests in `tests/metar_modifier.rs` covering `SPECI` parsing, `SPECI COR`,
  `SPECI NIL`, and describe output for both types.

- Extended `tests/metar_runway_state.rs` with 9 new parsing tests covering: `R/SNOCLO`,
  designators with `L`/`R`/`C` suffixes, all data positions as `/` (all fields `None`),
  individual missing fields (thickness or braking action), multiple runway state tokens
  in a single METAR, and rejection of invalid suffixes.
- Added `tests/describe_runway_state.rs` with 15 tests covering: SNOCLO description,
  all-fields-missing (`"not reported"`), deposit 0 (*clear and dry*), coverage codes 1
  and 2, thickness codes `00`/`92`/`98`/`99`, braking action as µ coefficient, braking
  action codes `91`/`93`/`95`/`99`, and designators with suffix.

### Changed — **Breaking**

- `Metar` struct gains four new fields: `report_type`, `color_code`, `color_code_forecast`,
  `sea_state`, `wind_shear`. Code constructing `Metar` literals must add these fields.
- `MetarDescription` gains `report_type: String`. Code destructuring `MetarDescription`
  must add this field.
- Bumped crate version to `0.4.5`.
- README and CHANGELOG updated.

---

## [0.4.3] - 2026-03-17

### Added

- Added `WindVariation { min: u16, max: u16 }` struct to `metar::models::wind`.
- Added `variation: Option<WindVariation>` field to `Wind`; set when a `dddVddd` token
  follows the main wind group in a METAR (e.g. `180V240`).
- Added `parse_wind_variation(token: &str) -> Option<WindVariation>` to `metar::parser::wind`.
- The `describe_wind` helper now appends `", variable Xto Y°"` when `variation` is present.
- Added `taf::models::icing` module with:
  - `IcingIntensity` enum (None, Light, ModerateMixedOrRime, ModerateGlaze, Severe, Unknown).
  - `Icing { intensity, base_ft, thickness_ft }` struct.
- Added `taf::models::turbulence` module with:
  - `TurbulenceIntensity` enum (None, Light, ModerateInCloud, ModerateClearAir,
    SevereInCloud, SevereClearAir, Extreme, Unknown).
  - `Turbulence { intensity, base_ft, thickness_ft }` struct.
- TAF forecast parser now recognises `6ABBBC` (icing) and `5ABBBC` (turbulence) groups,
  populating `TafForecast::icing` and `TafForecast::turbulence` (`Vec`) accordingly.
- Added `describe_icing` and `describe_turbulence` helpers in `common::describe::fields`.
- Added `icing: Vec<String>` and `turbulence: Vec<String>` to `ForecastDescription`;
  printed by `Display` and `format_taf`.
- Added 6 integration tests in `tests/metar_wind_variation.rs`.
- Added 14 integration tests in `tests/taf_icing_turbulence.rs`.

### Changed — **Breaking**

- `RunwayState::contamination_extent` renamed to `RunwayState::coverage`.
- `RunwayState::deposit_depth` renamed to `RunwayState::thickness`.
  - Code accessing these fields must be updated accordingly.
- `Wind` struct gains `variation: Option<WindVariation>` — struct literal construction
  must add the new field (e.g. `variation: None`).
- `TafForecast` gains `icing: Vec<Icing>` and `turbulence: Vec<Turbulence>` — struct
  literal construction must include these fields.
- Bumped crate version to `0.4.3`.
- README and CHANGELOG updated.

---

## [0.4.2] - 2026-03-17

### Added

- Added `metar::models::remark` module with:
  - `Remark` enum covering the most common NWS/ICAO remark groups:
    `PeakWind`, `WindShift`, `SeaLevelPressure`, `PrecipitationAmount`,
    `HourlyTemperature`, `MaxMinTemperature`, `PressureTendency`,
    `AutoStation`, `Lightning`, `MaintenanceIndicator`, `Virga`,
    `PressureRisingRapidly`, `PressureFallingRapidly`, `SensorStatus`.
  - `Remarks` struct with `items: Vec<Remark>`, `unparsed: Vec<String>`,
    `raw: String`; implements `Default`.
  - `AutoStationKind` enum (`AO1` / `AO2`).
  - `LightningType` enum (`IC`, `CC`, `CA`, `CG`).
- Replaced trivial `parse_rmk` with a structured multi-token parser that:
  - handles two-token groups (`PK WND dddff/HHmm`, `WSHFT HHmm [FROPA]`);
  - parses single-token groups by format recognition (no regex);
  - places unrecognised tokens verbatim in `Remarks::unparsed`.
- Added `nosig: bool` field to `Metar`, set to `true` when a `NOSIG` trend is present.
- Added `describe_remarks(remarks: &Remarks) -> Option<String>` helper in
  `common::describe::fields`, used automatically by `describe_metar`.
- Added 22 integration tests in `tests/metar_remark_parser.rs` covering all
  recognised remark variants, multi-remark sections, and unknown-token fallback.

### Changed — **Breaking**

- `Metar::rmk: Option<String>` replaced by `Metar::remarks: Remarks`.
  - Raw text is still accessible via `metar.remarks.raw`.
  - Code using `metar.rmk` must be updated to `metar.remarks.raw`.
- `tests/metar_rmk.rs` updated to use `remarks.raw` instead of `rmk`.
- Bumped crate version to `0.4.2`.
- README updated with structured remark access example and dependency version.

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
- Implemented `std::fmt::Display` for `MetarDescription`, `ForecastDescription`, and `TafDescription`:
  a single `println!("{}", desc)` prints the full human-readable report, skipping absent fields.
- Added `format_metar(metar: &Metar, lang: Language) -> String` convenience function: parses,
  describes, and formats in one call.
- Added `format_taf(taf: &Taf, lang: Language) -> String` convenience function: same for TAF.
- Re-exported `format_metar` and `format_taf` from `lib.rs`.
- Added 17 integration tests in `tests/format_describe.rs` covering `format_metar`, `format_taf`,
  `Display` output correctness, absent-field omission, and multi-group rendering.

### Changed

- Bumped crate version to `0.4.0`.
- README updated with `describe_metar`, `describe_taf`, `format_metar`, and `format_taf` usage
  examples and dependency snippet.

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
