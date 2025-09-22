# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]
- Improved error messages in codegen
- Add support for auto-running `i18n-codegen` from a build script
- Add CI workflow for publishing crates

---

## [0.1.0] - 2025-09-22
### Added
- **i18n-runtime**
  - Runtime JSON loader: `I18n::from_json_dir`
  - Compile-time registry loader: `I18n::from_generated_registry`
  - Locale fallback logic (`en-IN-BR → en-IN → en`)
  - `Locale` type with tag normalization
  - Basic translation lookup (`.get_owned`, `.get_by_str_key`)
- **i18n-codegen**
  - CLI tool `i18n-codegen` (`cargo install i18n-codegen`)
  - Generates:
    - `MessageKey` enum from `messages.schema.json`
    - One `*.rs` file per locale JSON (`locales/en.json → EN.rs`, etc.)
    - A `mod.rs` registry with `GENERATED_REGISTRY`
  - Strict validation (missing or extra keys cause build failure)
- **i18n-app (demo)**
  - Example project showing both runtime JSON mode and generated PHF mode
  - Demonstrates fallback logic and safe `MessageKey` usage

---

## [0.0.1] - 2025-09-20 (internal only)
- Initial experiments with PHF maps and locale fallback
- Basic build.rs generator (superseded by `i18n-codegen`)
- Not published to crates.io
