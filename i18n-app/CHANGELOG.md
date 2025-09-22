# Changelog — i18n-app (Demo)
This is a demo application crate showing how to use `i18n-runtime` and `i18n-codegen`.

---

## [Unreleased]
- Planned: Add example for using fallback locales in-depth.
- Planned: Add example for pluralization (once supported in runtime).
- Planned: Add CI example for running `i18n-codegen` automatically.

---

## [0.1.0] - 2025-09-22
### Added
- Demonstration of **runtime JSON mode**:
  - Loads from `./locales` at runtime with fallback `"en"`.
- Demonstration of **generated PHF mode**:
  - Uses `i18n-codegen` output in `src/generated_i18n/`.
  - Shows how to include `MessageKey` and registry.
- Prints translations for multiple locales (`en`, `hi-IN`, `fr`).
- Example fallback resolution: `fr` falls back to `"en"`.
