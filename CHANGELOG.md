# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2023-07-20

- [#6]: Update dependencies and release `v0.2.0`
- [#4]: Derive Copy+Clone for Interrupt

[#6]: https://github.com/japaric/lm3s6965/pull/6
[#4]: https://github.com/japaric/lm3s6965/pull/4

## [v0.1.3] - 2018-10-30

### Fixed

- [#3]: `thumbv6m-none-eabi` support. When compiled for that target this crate will
  only expose 32 interrupts, which is the maximum number of interrupts that
  `cortex-m-rt` allows for the ARMv6-M architecture.

[#3]: https://github.com/japaric/lm3s6965/pull/3

## [v0.1.2] - 2018-10-30

### Changed

- [2a92b7a]: Inlined a few functions

[2a92b7a]: https://github.com/japaric/lm3s6965/commit/2a92b7a0e75efacc048d658839ed927dd35fa415

## [v0.1.1] - 2018-10-24

### Added

- [#2]: Ship a memory.x file describing the memory layout of the LM3S6965.

[#2]: https://github.com/japaric/lm3s6965/pull/2

## v0.1.0 - 2018-10-24

Initial release

[Unreleased]: https://github.com/japaric/lm3s6965/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/lm3s6965/compare/v0.1.3...v0.2.0
[v0.1.3]: https://github.com/japaric/lm3s6965/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/lm3s6965/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/lm3s6965/compare/v0.1.0...v0.1.1
