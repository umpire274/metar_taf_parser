# Python fork regex-alignment policy

To reduce behavioral drift during the Rust port, each METAR/TAF group parser should be migrated
using the same regex definitions used by the Mivek's Python original project (`mivek/python-metar-taf-parser`).

## Rules

1. Start each parser module from the Python regex patterns (same token semantics).
2. Keep tests in Rust that mirror Python examples for every regex group.
3. Port incrementally by group (visibility, wind, clouds, weather, pressure, trend, TAF forecast parts).
4. Only refactor regexes after parity tests are green.

## Immediate next modules

- METAR runway visual range (RVR) groups
- METAR advanced trend payloads beyond marker tokens (`BECMG`/`TEMPO` details)
- TAF temperature groups (`TX`/`TN`) and optional wind-shear sections
