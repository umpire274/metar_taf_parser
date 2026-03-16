# Codebase Reference (Complete Inventory)

This document provides an English reference for the current Rust codebase, including modules, structs, enums, and functions (public and internal helpers).

> Porting status note: baseline parser parity is complete as of `v0.3.0`; see `docs/PORTING_REGEX_POLICY.md` for maintenance guidance.


## Module Map


### `src/airports/mod.rs`

- `pub mod db`: Exposes the `db` module to its parent scope.
- `pub mod model`: Exposes the `model` module to its parent scope.

### `src/common/mod.rs`

- `pub mod report_modifier`: Exposes the `report_modifier` module to its parent scope.
- `pub mod tokenizer`: Exposes the `tokenizer` module to its parent scope.
- `pub mod units`: Exposes the `units` module to its parent scope.
- `pub mod weather_codes`: Exposes the `weather_codes` module to its parent scope.

### `src/lib.rs`

- `pub mod airports`: Exposes the `airports` module to its parent scope.
- `pub mod common`: Exposes the `common` module to its parent scope.
- `pub mod metar`: Exposes the `metar` module to its parent scope.
- `pub mod taf`: Exposes the `taf` module to its parent scope.

### `src/metar/mod.rs`

- `pub mod errors`: Exposes the `errors` module to its parent scope.
- `pub mod models`: Exposes the `models` module to its parent scope.
- `pub mod parser`: Exposes the `parser` module to its parent scope.

### `src/metar/models/mod.rs`

- `pub mod cloud`: Exposes the `cloud` module to its parent scope.
- `pub mod metar`: Exposes the `metar` module to its parent scope.
- `pub mod pressure`: Exposes the `pressure` module to its parent scope.
- `pub mod rvr`: Exposes the `rvr` module to its parent scope.
- `pub mod temperature`: Exposes the `temperature` module to its parent scope.
- `pub mod time`: Exposes the `time` module to its parent scope.
- `pub mod trend`: Exposes the `trend` module to its parent scope.
- `pub mod visibility`: Exposes the `visibility` module to its parent scope.
- `pub mod weather`: Exposes the `weather` module to its parent scope.
- `pub mod wind`: Exposes the `wind` module to its parent scope.

### `src/metar/parser/mod.rs`

- `pub mod cloud`: Exposes the `cloud` module to its parent scope.
- `pub mod metar`: Exposes the `metar` module to its parent scope.
- `pub mod pressure`: Exposes the `pressure` module to its parent scope.
- `pub mod rmk`: Exposes the `rmk` module to its parent scope.
- `pub mod runway_state`: Exposes the `runway_state` module to its parent scope.
- `pub mod rvr`: Exposes the `rvr` module to its parent scope.
- `pub mod temperature`: Exposes the `temperature` module to its parent scope.
- `pub mod time`: Exposes the `time` module to its parent scope.
- `pub mod trend`: Exposes the `trend` module to its parent scope.
- `pub mod visibility`: Exposes the `visibility` module to its parent scope.
- `pub mod weather`: Exposes the `weather` module to its parent scope.
- `pub mod wind`: Exposes the `wind` module to its parent scope.

### `src/taf/mod.rs`

- `pub mod errors`: Exposes the `errors` module to its parent scope.
- `pub mod models`: Exposes the `models` module to its parent scope.
- `pub mod parser`: Exposes the `parser` module to its parent scope.

### `src/taf/models/mod.rs`

- `pub mod forecast`: Exposes the `forecast` module to its parent scope.
- `pub mod taf`: Exposes the `taf` module to its parent scope.
- `pub mod temperature`: Exposes the `temperature` module to its parent scope.
- `pub mod time`: Exposes the `time` module to its parent scope.
- `pub mod wind_shear`: Exposes the `wind_shear` module to its parent scope.

### `src/taf/parser/mod.rs`

- `pub mod forecast`: Exposes the `forecast` module to its parent scope.
- `pub mod taf`: Exposes the `taf` module to its parent scope.
- `pub mod time`: Exposes the `time` module to its parent scope.

## Item Inventory


### `src/airports/db.rs`

- `AirportDb` (public struct, line 5): Data container used to store and query structured information.
- `load` (public fn, line 10): Internal helper function used by parsing or model assembly logic.
- `lookup` (public fn, line 24): Lookup helper used to query known data by key.
- `parse_airport_line` (internal fn, line 30): Parser function that validates and converts tokens into typed model data.
- `split_csv_line` (internal fn, line 62): String/token normalization helper used during parsing.
- `lookup_known_airport` (internal fn, line 90): Lookup helper used to query known data by key.

### `src/airports/model.rs`

- `Airport` (public struct, line 4): Structured domain model used by parsing/output code.

### `src/common/report_modifier.rs`

- `ReportModifier` (public enum, line 4): Enumerated domain values used to represent constrained parser/model states.

### `src/common/tokenizer.rs`

- `Tokenizer` (public struct, line 2): Structured domain model used by parsing/output code.
- `new` (public fn, line 8): Constructor/helper that builds a model with normalized defaults.
- `peek` (public fn, line 19): Internal helper function used by parsing or model assembly logic.
- `Item` (internal type, line 25): Type alias used to simplify iterator or model signatures.
- `next` (internal fn, line 27): Internal helper function used by parsing or model assembly logic.

### `src/metar/errors.rs`

- `MetarError` (public enum, line 4): Enumerated domain values used to represent constrained parser/model states.

### `src/metar/models/cloud.rs`

- `CloudLayer` (public struct, line 4): Structured domain model used by parsing/output code.
- `CloudAmount` (public enum, line 11): Enumerated domain values used to represent constrained parser/model states.
- `CloudType` (public enum, line 22): Enumerated domain values used to represent constrained parser/model states.

### `src/metar/models/metar.rs`

- `Metar` (public struct, line 14): Top-level parsed report model.
- `new` (public fn, line 33): Constructor/helper that builds a model with normalized defaults.

### `src/metar/models/pressure.rs`

- `Pressure` (public enum, line 4): Enumerated domain values used to represent constrained parser/model states.

### `src/metar/models/runway_state.rs`

- `RunwayState` (public struct, line 4): Structured domain model used by parsing/output code.

### `src/metar/models/rvr.rs`

- `RvrQualifier` (public enum, line 4): Enumerated domain values used to represent constrained parser/model states.
- `RvrTendency` (public enum, line 10): Enumerated domain values used to represent constrained parser/model states.
- `RvrUnit` (public enum, line 17): Enumerated domain values used to represent constrained parser/model states.
- `RvrValue` (public struct, line 23): Structured domain model used by parsing/output code.
- `RunwayVisualRange` (public struct, line 29): Structured domain model used by parsing/output code.

### `src/metar/models/temperature.rs`

- `Temperature` (public struct, line 4): Structured temperature-related model carrying parsed values.

### `src/metar/models/time.rs`

- `MetarTime` (public struct, line 4): Structured time value used by parsers and domain models.

### `src/metar/models/trend.rs`

- `MetarTrend` (public enum, line 4): Enumerated domain values used to represent constrained parser/model states.

### `src/metar/models/visibility.rs`

- `Visibility` (public enum, line 4): Enumerated domain values used to represent constrained parser/model states.
- `VisibilityDirection` (public enum, line 17): Enumerated domain values used to represent constrained parser/model states.

### `src/metar/models/weather.rs`

- `WeatherIntensity` (public enum, line 4): Enumerated domain values used to represent constrained parser/model states.
- `WeatherDescriptor` (public enum, line 11): Enumerated domain values used to represent constrained parser/model states.
- `WeatherPhenomenon` (public enum, line 24): Enumerated domain values used to represent constrained parser/model states.
- `Weather` (public struct, line 39): Structured domain model used by parsing/output code.

### `src/metar/models/wind.rs`

- `WindUnit` (public enum, line 4): Enumerated domain values used to represent constrained parser/model states.
- `Wind` (public struct, line 10): Structured wind-related model carrying parsed meteorological fields.

### `src/metar/parser/cloud.rs`

- `parse_cloud` (public fn, line 3): Parser function that validates and converts tokens into typed model data.
- `parse_cloud_amount` (internal fn, line 77): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/metar.rs`

- `parse_metar` (public fn, line 17): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/pressure.rs`

- `parse_pressure` (public fn, line 3): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/rmk.rs`

- `parse_rmk` (public fn, line 1): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/runway_state.rs`

- `parse_runway_state` (public fn, line 3): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/rvr.rs`

- `parse_rvr` (public fn, line 3): Parser function that validates and converts tokens into typed model data.
- `is_valid_runway_designator` (internal fn, line 39): Internal helper function used by parsing or model assembly logic.
- `parse_tendency` (internal fn, line 56): Parser function that validates and converts tokens into typed model data.
- `parse_unit` (internal fn, line 74): Parser function that validates and converts tokens into typed model data.
- `parse_rvr_value` (internal fn, line 82): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/temperature.rs`

- `parse_single_temp` (internal fn, line 3): Parser function that validates and converts tokens into typed model data.
- `parse_temperature` (public fn, line 32): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/time.rs`

- `parse_time` (public fn, line 3): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/trend.rs`

- `parse_trend` (public fn, line 3): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/visibility.rs`

- `METERS_PER_STATUTE_MILE` (internal const, line 4): Constant used by parser conversion/validation logic.
- `parse_visibility` (public fn, line 6): Parser function that validates and converts tokens into typed model data.
- `parse_split_statute_miles_to_meters` (public fn, line 42): Parser function that validates and converts tokens into typed model data.
- `parse_statute_miles_to_meters` (internal fn, line 61): Parser function that validates and converts tokens into typed model data.
- `parse_fraction` (internal fn, line 76): Parser function that validates and converts tokens into typed model data.
- `miles_to_meters` (internal fn, line 88): Internal helper function used by parsing or model assembly logic.
- `parse_visibility_direction` (internal fn, line 102): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/weather.rs`

- `parse_weather` (public fn, line 3): Parser function that validates and converts tokens into typed model data.

### `src/metar/parser/wind.rs`

- `strip_unit` (internal fn, line 3): String/token normalization helper used during parsing.
- `parse_wind` (public fn, line 13): Parser function that validates and converts tokens into typed model data.

### `src/taf/errors.rs`

- `TafError` (public enum, line 4): Enumerated domain values used to represent constrained parser/model states.

### `src/taf/models/forecast.rs`

- `TafForecastKind` (public enum, line 11): Enumerated domain values used to represent constrained parser/model states.
- `TafForecast` (public struct, line 20): Structured forecast model for a TAF forecast section.

### `src/taf/models/taf.rs`

- `Taf` (public struct, line 7): Top-level parsed report model.

### `src/taf/models/temperature.rs`

- `TafTemperature` (public struct, line 4): Structured temperature-related model carrying parsed values.

### `src/taf/models/time.rs`

- `TafTime` (public struct, line 4): Structured time value used by parsers and domain models.
- `TafValidity` (public struct, line 11): Structured time value used by parsers and domain models.
- `TafPeriod` (public struct, line 18): Structured time value used by parsers and domain models.

### `src/taf/models/wind_shear.rs`

- `TafWindShear` (public struct, line 4): Structured wind-related model carrying parsed meteorological fields.

### `src/taf/parser/forecast.rs`

- `parse_forecasts` (public fn, line 16): Parser function that validates and converts tokens into typed model data.
- `new_base_forecast` (internal fn, line 125): Constructor/helper that builds a model with normalized defaults.
- `new_fm_forecast` (internal fn, line 141): Constructor/helper that builds a model with normalized defaults.
- `new_period_forecast` (internal fn, line 157): Constructor/helper that builds a model with normalized defaults.
- `parse_fm_time` (internal fn, line 179): Parser function that validates and converts tokens into typed model data.
- `parse_becmg_period` (internal fn, line 187): Parser function that validates and converts tokens into typed model data.
- `parse_prob` (internal fn, line 199): Parser function that validates and converts tokens into typed model data.
- `try_consume_prob_period` (internal fn, line 207): Internal helper function used by parsing or model assembly logic.
- `try_consume_period` (internal fn, line 227): Internal helper function used by parsing or model assembly logic.
- `parse_day_hour` (internal fn, line 235): Parser function that validates and converts tokens into typed model data.
- `parse_day_hour_min` (internal fn, line 250): Parser function that validates and converts tokens into typed model data.
- `parse_taf_wind_shear` (internal fn, line 266): Parser function that validates and converts tokens into typed model data.
- `parse_taf_temperature` (internal fn, line 294): Parser function that validates and converts tokens into typed model data.
- `parse_signed_temp` (internal fn, line 326): Parser function that validates and converts tokens into typed model data.
- `fake_metar` (internal fn, line 345): Internal helper function used by parsing or model assembly logic.

### `src/taf/parser/taf.rs`

- `parse_taf` (public fn, line 7): Parser function that validates and converts tokens into typed model data.
- `parse_taf_strict` (public fn, line 11): Parser function that validates and converts tokens into typed model data.
- `parse_taf_with_mode` (internal fn, line 15): Parser function that validates and converts tokens into typed model data.

### `src/taf/parser/time.rs`

- `parse_taf_time` (public fn, line 4): Parser function that validates and converts tokens into typed model data.
- `parse_validity` (public fn, line 21): Parser function that validates and converts tokens into typed model data.
