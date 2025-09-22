# Changelog — i18n-runtime
All notable changes to this crate will be documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),  
and this crate adheres to [Semantic Versioning](https://semver.org/).

---

## [Unreleased]
- Planned: Add macro-based API for ergonomic `t!(...)` lookups.
- Planned: Add pluralization support.
- Planned: Add `no_std` support for embedded systems.

---

## [0.1.0] - 2025-09-22
### Added
- `I18n::from_json_dir` — load translations dynamically from JSON files.
- `I18n::from_generated_registry` — load translations from PHF-generated maps.
- `Locale` type with fallback resolution:
  - Handles `en-IN-BR → en-IN → en → fallback`.
- Lookup methods:
  - `.get_owned(&Locale, &str)` — runtime string lookups.
  - `.get_by_str_key(&Locale, &str)` — type-safe lookups via generated keys.
- Error handling: clear messages when keys or locales are missing.
