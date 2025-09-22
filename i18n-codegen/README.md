# ⚡ i18n-codegen

A CLI tool that generates **fast, type-safe Rust code** for translations.  
Designed to work seamlessly with [i18n-runtime](https://crates.io/crates/i18n-runtime).

---

## ✨ Features
- 🔧 Reads `messages.schema.json` + `locales/*.json`
- 🔑 Generates a `MessageKey` enum (type-safe keys)
- ⚡ Produces [`phf`](https://crates.io/crates/phf) maps for O(1) lookups
- 📂 Outputs clean Rust files into `src/generated_i18n/`
- 🔄 Works with `i18n-runtime`’s fallback logic (`en-IN-BR → en-IN → en`)

---

## 📦 Installation

```bash
cargo install i18n-codegen
````

---

## 🚀 Usage

1. Define your schema in `messages.schema.json`:

```json
{
  "keys": ["welcome", "login_success", "login_failed"]
}
```

2. Add locale files in `locales/`. Example `locales/en.json`:

```json
{
  "welcome": "Welcome!",
  "login_success": "You have logged in successfully.",
  "login_failed": "Login failed. Please try again."
}
```

3. Run the generator:

```bash
i18n-codegen ./ ./src/generated_i18n
```

4. Generated files:

```
src/generated_i18n/generated_keys.rs   # MessageKey enum
src/generated_i18n/locales/EN.rs       # en.json → phf::Map
src/generated_i18n/locales/HI_IN.rs    # hi-IN.json → phf::Map
src/generated_i18n/mod.rs              # registry
```

---

## 📦 Integration in your app

In `src/main.rs`:

```rust
// include generated code
include!("generated_i18n/generated_keys.rs");
mod generated_i18n { include!("generated_i18n/mod.rs"); }

use i18n_runtime::{I18n, Locale};

fn main() {
    let registry = generated_i18n::get_generated_registry();
    let i18n = I18n::from_generated_registry(registry, "en");

    let msg = i18n
        .get_by_str_key(&Locale::new("en"), MessageKey::Welcome.as_str())
        .unwrap();
    println!("{}", msg); // Welcome!
}
```

---

## 🔄 Workflow

1. Edit your JSON files in `locales/`
2. Re-run `i18n-codegen` to regenerate Rust code
3. Rebuild your app → translations are now compiled in and lightning-fast ⚡

---

## ❓ FAQ

**Q: Do I need this tool?**
➡️ Only if you want **compile-time PHF mode** for maximum performance.
For dynamic translation loading, use `i18n-runtime` alone.

**Q: Should I check in generated files to git?**
➡️ Yes, recommended. Makes builds reproducible and avoids requiring the generator on every machine.

**Q: Can I run it automatically in `build.rs`?**
➡️ Yes! Just call `i18n-codegen` from your build script. Or run it manually in CI.

---

## 🧩 Ecosystem

* [**i18n-runtime**](https://crates.io/crates/i18n-runtime) — runtime API
* [**i18n-codegen**](https://crates.io/crates/i18n-codegen) — generator CLI (this crate)

---

## 📜 License

MIT

---