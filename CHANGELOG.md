# Changelog

## [unreleased]

### Added

- Add CLI.
- Add deserialization support.
- Add `mcnbt::tag::Tag::pretty` function.
- Add `mcnbt::tag::Tag::name` function.
- Add `mcnbt::tag::INDENT` constant.
- Add `mcnbt::tag::ABBREVIATE_ARRAY_SIZE` constant.

### Changed

- Improve errors.

### Fixed

- Empty lists now use end tag as the type.


## [1.0.0] - 2023-10-27

### Added

- Add several helper macros for creating tags.
- Add discriminants to `mcnbt::Tag`.
- Add `mcnbt::errors` module.

### Changed

- **BREAKING**: `mcnbt::tag::Tag::as_bytes` has been renamed to `to_bytes` and now
  returns a `Result`.
- **BREAKING**: Changed types of `mcnbt::tag::Tag` variants.

### Removed

- **BREAKING**: Remove `mcnbt::FORMAT_VERSION`.


## [0.2.0] - 2023-10-21

### Added

- Add `nbt` macro.
- Add common traits for `mcnbt::tag::Tag`.
- Add `mcnbt::byte_order::ByteOrder::bytes`.


## [0.1.0] - 2023-07-02

_🍰 Initial release_


[unreleased]: https://github.com/phoenixr-codes/mcnbt/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/phoenixr-codes/mcnbt/compare/v0.2.0...v1.0.0
[0.2.0]: https://github.com/phoenixr-codes/mcnbt/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/phoenixr-codes/mcnbt/releases/v0.1.0