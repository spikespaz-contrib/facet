# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.22.0](https://github.com/facet-rs/facet/compare/facet-serialize-v0.21.0...facet-serialize-v0.22.0) - 2025-05-06

### Added

- *(serialize)* add more optional start- and end-serialize calls

### Fixed

- *(reflect)* add missing scalar types

### Other

- *(serialize)* make end_* trait methods optional
- *(serialize)* optionally widen-cast number types to u64 in trait

## [0.21.0](https://github.com/facet-rs/facet/compare/facet-serialize-v0.20.0...facet-serialize-v0.21.0) - 2025-05-02

### Other

- JSON facet-serialize?
- Use facet-serialize in facet-msgpack
- Don't depend on git version of insta anymore
- Introduce facet-serialize
