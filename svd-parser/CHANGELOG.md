# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.14.8] - 2025-02-08

- Revert the `riscv` element, as well as the `unstable-riscv` feature.

## [v0.14.7] - 2024-10-03

- Bump svd-rs to 0.14.9

## [v0.14.6] - 2024-08-20

- Adapt the `riscv` element to handle `riscv::Exception`.
- Add `riscv` element for configuration parameters related to RISC-V targets.
  You must use the `unstable-riscv` feature to enable this exeperimental element.
- Bump MSRV to 1.65.0
- Bump roxmltree to 0.20

## [v0.14.5] - 2024-01-03

- Bump MSRV to 1.61.0
- Bump svd-rs to 0.14.7, roxmltree to 0.19

## [v0.14.4] - 2023-11-15

- Bump svd-rs dependency to 0.14.4 or higher.

## [v0.14.3] - 2023-11-15

- Correctly place `expand_properties` under `expand` feature

## [v0.14.2] - 2023-09-17

- Bump MSRV to 1.58.0
- Ignore whitespaces around tag contents

## [v0.14.1] - 2022-10-23

- Update to `svd-rs` 0.14.1
- Add `BlockPath::parent`

## [v0.14.0] - 2022-07-19

- Make `expand::Index`, pathes & `derive_peripheral`, etc. public
- Fix parsing `xs:noNamespaceSchemaLocation`
- Bump MSRV to 1.56.0 (2021)

## [v0.13.4] - 2022-05-13

- Support nested `derivedFrom` for `expand`

## [v0.13.3] - 2022-05-09

- Add `expand_properties` (under `expand` feature)

## [v0.13.2] - 2022-04-23

- Add `expand` (under `expand` feature) and `ignore_enums` options

## [v0.13.1] - 2022-01-04

- Make `version`, `description`, `width` and `address_unit_bits` on `Device` optional again

## [v0.13.0] - 2022-01-04

- Add `svd2yaml` example
- Bump `svd-rs`

## [v0.12.0] - 2021-11-11

- Bump `svd-rs`
- Add `protection` parsing
- Add `readAction` parsing
- Add array support for peripherals

## [v0.11.0] - 2021-10-02

Previous versions in common [changelog](../CHANGELOG.md).

[Unreleased]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.10...HEAD
[v0.14.8]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.9...svd-rs-v0.14.10
[v0.14.6]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.8...svd-rs-v0.14.9
[v0.14.5]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.14.4...svd-rs-v0.14.7
[v0.14.4]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.14.3...svd-parser-v0.14.4
[v0.14.3]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.14.2...svd-parser-v0.14.3
[v0.14.2]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.14.2...svd-parser-v0.14.2
[v0.14.1]: https://github.com/rust-embedded/svd/compare/v0.14.0...svd-rs-v0.14.1
[v0.14.0]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.13.4...v0.14.0
[v0.13.4]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.13.3...svd-parser-v0.13.4
[v0.13.3]: https://github.com/rust-embedded/svd/compare/svd-parser-v0.13.2...svd-parser-v0.13.3
[v0.13.2]: https://github.com/rust-embedded/svd/compare/svd-rs-v0.13.2...svd-parser-v0.13.2
[v0.13.1]: https://github.com/rust-embedded/svd/compare/v0.13.0...svd-parser-v0.13.1
[v0.13.0]: https://github.com/rust-embedded/svd/compare/v0.12.0...v0.13.0
[v0.12.0]: https://github.com/rust-embedded/svd/compare/v0.11.0...v0.12.0
[v0.11.0]: https://github.com/rust-embedded/svd/compare/v0.10.2...v0.11.0
