# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0-alpha.9] - 2026-06-09

### Changed

- RelationType renamed to SituationRelationType

## [0.1.0-alpha.8] - 2026-06-09

### Added

- SituationRelation type with RelationType and Confidence enums

## [0.1.0-alpha.5] - 2026-06-09

### Added

- Domain type: name + label pair

## [0.1.0-alpha.4] - 2026-06-09

### Removed

- ser module; users call serde directly

## [0.1.0-alpha.3] - 2026-06-09

### Changed

- serde_json made unconditional
- add Pydantic-style ser module (ToJson/FromJson/ToYaml/FromYaml)

## [0.1.0-alpha.2] - 2026-06-09

### Changed

- fix publish workflow: use CRATES_API_TOKEN secret name

## [0.1.0-alpha.1] - 2026-06-09

### Added

- Rust workspace with four data model crates (thought, intention, situation, schema)
- integration tests for all four models
- CI workflow (rust-build): cargo build + test on push/PR
- publish workflow (rust-publish): version validation + crates.io publish
