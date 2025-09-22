# Changelog — i18n-codegen
All notable changes to this crate will be documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),  
and this crate adheres to [Semantic Versioning](https://semver.org/).

---

## [Unreleased]
- Planned: Add watch mode (`--watch`) to regenerate on file changes.
- Planned: Add YAML/TOML locale file support.
- Planned: Add option to generate Rust `const fn` API.

---

## [0.1.0] - 2025-09-22
### Added
- CLI tool `i18n-codegen` (`cargo install i18n-codegen`).
- Generates:
  - `generated_keys.rs` — defines `MessageKey` enum from `messages.schema.json`.
  - One `*.rs` file per locale (`locales/en.json → EN.rs`, `locales/hi-IN.json → HI_IN.rs`, etc.).
  - `mod.rs` registry with `pub static GENERATED_REGISTRY`.
- Strict schema enforcement:
  - Build fails if locale files are missing keys.
  - Build fails if locale files contain extra keys.
- Output is clean, compiler-checked, and integrates with `i18n-runtime`.
