# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.1.3] - 2018-10-30

### Fixed

- `thumbv6m-none-eabi` support. When compiled for that target this crate will
  only expose 32 interrupts, which is the maximum number of interrupts that
  `cortex-m-rt` allows for the ARMv6-M architecture.

## [v0.1.2] - 2018-10-30

### Changed

- Inlined a few functions

## [v0.1.1] - 2018-10-24

### Added

- Ship a memory.x file describing the memory layout of the LM3S6965.

## v0.1.0 - 2018-10-24

Initial release

[Unreleased]: https://github.com/japaric/lm3s6965/compare/v0.1.2...HEAD
[v0.1.2]: https://github.com/japaric/lm3s6965/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/lm3s6965/compare/v0.1.0...v0.1.1
