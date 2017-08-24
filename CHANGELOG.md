# Change Log

All releases of **static_assertions** adhere to [Semantic Versioning][semver].

---

## [v0.2.2](https://github.com/nvzqz/static-assertions-rs/tree/v0.2.2) (2017-08-13)

- [Changes](https://github.com/nvzqz/static-assertions-rs/compare/v0.2.1...v0.2.2)
- [Release](https://github.com/nvzqz/static-assertions-rs/releases/tag/v0.2.2)

### New Features
- Added `assert_impl` macro to ensure a type implements a given set of traits

---

## [v0.2.1](https://github.com/nvzqz/static-assertions-rs/tree/v0.2.1) (2017-08-13)

- [Changes](https://github.com/nvzqz/static-assertions-rs/compare/v0.2.0...v0.2.1)
- [Release](https://github.com/nvzqz/static-assertions-rs/releases/tag/v0.2.1)

### New Features
- Added `assert_obj_safe` macro for ensuring that a trait is object-safe

---

## [v0.2.0](https://github.com/nvzqz/static-assertions-rs/tree/v0.2.0) (2017-08-12)

- [Changes](https://github.com/nvzqz/static-assertions-rs/compare/v0.1.1...v0.2.0)
- [Release](https://github.com/nvzqz/static-assertions-rs/releases/tag/v0.2.0)

### New Features
- Added `assert_eq_size_ptr` macro

### Improvements
- Allow `assert_eq_size`, `const_assert`, and `const_assert_eq` in non-function contexts via providing a unique label #1

### Changes
- Semicolon-separated `assert_eq_size` is no longer allowed

---

## [v0.1.1](https://github.com/nvzqz/static-assertions-rs/tree/v0.1.1) (2017-08-12)

- [Changes](https://github.com/nvzqz/static-assertions-rs/compare/v0.1.0...v0.1.1)
- [Release](https://github.com/nvzqz/static-assertions-rs/releases/tag/v0.1.1)

### New Features
- Added `const_assert_eq` macro

---

## [v1.0.0](https://github.com/nvzqz/static-assertions-rs/tree/v1.0.0) (2017-08-12)

- [Release](https://github.com/nvzqz/static-assertions-rs/releases/tag/v1.0.0)

Initial release

[semver]: http://semver.org/
