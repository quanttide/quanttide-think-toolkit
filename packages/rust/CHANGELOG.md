# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0-alpha.10] - 2026-06-09

### Changed

- CI: add --allow-dirty to cargo publish (handle release journal)

## [0.1.0-alpha.9] - 2026-06-09

### Changed

- RelationType renamed to SituationRelationType

## [0.1.0-alpha.8] - 2026-06-09

### Added

- SituationRelation type: source, target, relation_type, confidence, description
- RelationType enum: support, conflict, trigger, evolve
- Confidence enum: high, low

## [0.1.0-alpha.5] - 2026-06-09

### Added

- Domain type: name + label pair for registry entries

## [0.1.0-alpha.4] - 2026-06-09

### Removed

- ser module: convenience traits removed; users call serde directly

### Changed

- remove feature flags; serde_json is unconditional, serde_yaml is dev-only

## [0.1.0-alpha.3] - 2026-06-09

### Changed

- serde_json made unconditional (required by Mapping.action: Value)
- json feature becomes marker; yaml feature controls serde_yaml
- add Pydantic-style ser module (ToJson/FromJson/ToYaml/FromYaml)

## [0.1.0-alpha.2] - 2026-06-09

### Changed

- fix publish workflow: use CRATES_API_TOKEN secret name

## [0.1.0-alpha.1] - 2026-06-09

### Added

- data models: Thought, Intention, Situation, Schema with serde derives
- integration tests for all four models (construct, roundtrip, gallery format)
- CI workflow (rust-build): cargo build + test on push/PR
- publish workflow (rust-publish): version validation + crates.io publish
