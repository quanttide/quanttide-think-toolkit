# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0-alpha.2] - 2026-06-09

### Changed

- fix publish workflow: use CRATES_API_TOKEN secret name

## [0.1.0-alpha.1] - 2026-06-09

### Added

- data models: Thought, Intention, Situation, Schema with serde derives
- integration tests for all four models (construct, roundtrip, gallery format)
- CI workflow (rust-build): cargo build + test on push/PR
- publish workflow (rust-publish): version validation + crates.io publish
