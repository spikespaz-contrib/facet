# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.24.9](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.24.8...facet-deserialize-v0.24.9) - 2025-05-20

### Added

- *(args)* arg-wise spans for reflection errors; ToCooked trait

### Other

- Show warning on truncation
- Truncate when showing errors in one long JSON line

## [0.24.8](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.24.7...facet-deserialize-v0.24.8) - 2025-05-18

### Other

- Introduce `'shape` lifetime, allowing non-'static shapes.

## [0.24.7](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.24.6...facet-deserialize-v0.24.7) - 2025-05-16

### Added

- facet-args `Cli` trait impl; deserialize `&str` field
- *(deserialize)* support multiple input types via generic `Format`

## [0.24.6](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.24.5...facet-deserialize-v0.24.6) - 2025-05-13

### Other

- Fix enum tests with a single tuple field
- Well it says the field is not initialized, so.

## [0.24.5](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.24.4...facet-deserialize-v0.24.5) - 2025-05-12

### Other

- updated the following local packages: facet-core, facet-reflect

## [0.24.4](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.24.3...facet-deserialize-v0.24.4) - 2025-05-12

### Added

- *(facet-args)* rely on facet-deserialize StackRunner

## [0.24.3](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.24.2...facet-deserialize-v0.24.3) - 2025-05-10

### Other

- updated the following local packages: facet-core, facet-reflect

## [0.24.2](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.24.1...facet-deserialize-v0.24.2) - 2025-05-10

### Other

- Add support for partially initializing arrays, closes #504

## [0.24.1](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.24.0...facet-deserialize-v0.24.1) - 2025-05-10

### Other

- updated the following local packages: facet-core, facet-reflect

## [0.24.0](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.23.0...facet-deserialize-v0.24.0) - 2025-05-10

### Other

- Release facet-reflect
- Release facet-derive-parser
- Upgrade facet-core
- Determine enum variant after default_from_fn
- Uncomment deserialize

## [0.23.0](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.22.0...facet-deserialize-v0.23.0) - 2025-05-08

### Other

- *(deserialize)* [**breaking**] make deserialize format stateful

## [0.22.0](https://github.com/facet-rs/facet/compare/facet-deserialize-v0.21.0...facet-deserialize-v0.22.0) - 2025-05-06

### Other

- Get started on an event-based approach to facet-deserialize ([#500](https://github.com/facet-rs/facet/pull/500))
