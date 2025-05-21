# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.24.5](https://github.com/facet-rs/facet/compare/facet-json-v0.24.4...facet-json-v0.24.5) - 2025-05-21

### Other

- Support deserializing to `Arc<T>`

## [0.24.4](https://github.com/facet-rs/facet/compare/facet-json-v0.24.3...facet-json-v0.24.4) - 2025-05-20

### Added

- *(args)* arg-wise spans for reflection errors; ToCooked trait

### Other

- cfg(not(miri))
- Show warning on truncation
- Truncate when showing errors in one long JSON line

## [0.24.3](https://github.com/facet-rs/facet/compare/facet-json-v0.24.2...facet-json-v0.24.3) - 2025-05-18

### Other

- Introduce `'shape` lifetime, allowing non-'static shapes.

## [0.24.2](https://github.com/facet-rs/facet/compare/facet-json-v0.24.1...facet-json-v0.24.2) - 2025-05-16

### Added

- facet-args `Cli` trait impl; deserialize `&str` field
- *(deserialize)* support multiple input types via generic `Format`

### Other

- Rust 1.87 clippy fixes
- Relax facet-json lifetime requirements
- Re-export DeserError, DeserErrorKind, etc.
- Fix msrv
- almost fix everything
- implement recursive serialize
- Use test attribute for facet-json tests
- Introduce facet_testhelpers::test attribute
- Fix json tests
- Clean tests up, re-enable json tests
- allow deserializing from number in JSON

## [0.24.1](https://github.com/facet-rs/facet/compare/facet-json-v0.24.0...facet-json-v0.24.1) - 2025-05-13

### Other

- Fix enum tests with a single tuple field
- Well it says the field is not initialized, so.

## [0.23.6](https://github.com/facet-rs/facet/compare/facet-json-v0.23.5...facet-json-v0.23.6) - 2025-05-12

### Other

- updated the following local packages: facet-core, facet-core, facet-reflect, facet-deserialize, facet-serialize

## [0.23.5](https://github.com/facet-rs/facet/compare/facet-json-v0.23.4...facet-json-v0.23.5) - 2025-05-12

### Added

- *(core)* add core implementation for `jiff::civil::DateTime`
- *(core)* add core implementation for `jiff::Timestamp`
- *(core)* add core implementation for `jiff::Zoned`

### Other

- Re-export DeserError
- Disable zoned test under miri
- Rename jiff feature to jiff02 (thanks @BurntSushi)
- Fix memory leaks, add more tests
- Add JSON test cases for Camino/ULID/UUID
- Add support for time crate's OffsetDateTime and UtcDateTime

## [0.23.4](https://github.com/facet-rs/facet/compare/facet-json-v0.23.3...facet-json-v0.23.4) - 2025-05-10

### Other

- updated the following local packages: facet-core, facet-reflect, facet-deserialize, facet-serialize

## [0.23.3](https://github.com/facet-rs/facet/compare/facet-json-v0.23.2...facet-json-v0.23.3) - 2025-05-10

### Other

- Add support for partially initializing arrays, closes #504

## [0.23.2](https://github.com/facet-rs/facet/compare/facet-json-v0.23.1...facet-json-v0.23.2) - 2025-05-10

### Other

- updated the following local packages: facet-core, facet-reflect, facet-deserialize, facet-serialize

## [0.23.1](https://github.com/facet-rs/facet/compare/facet-json-v0.23.0...facet-json-v0.23.1) - 2025-05-10

### Added

- Allow empty string rename values

### Fixed

- Add support for Unicode escape sequences in JSON strings

### Other

- Release facet-reflect
- Release facet-derive-parser
- Upgrade facet-core
- Fix additional tests
- Determine enum variant after default_from_fn
- Uncomment deserialize

## [0.23.0](https://github.com/facet-rs/facet/compare/facet-json-v0.22.0...facet-json-v0.23.0) - 2025-05-08

### Other

- *(deserialize)* [**breaking**] make deserialize format stateful
