# `i18n-runtime`** 🎉

---


# 🌍 i18n-runtime

Effortless, fast, and type-safe **internationalization (i18n)** for Rust apps.  
Supports both **runtime JSON loading** and **compile-time PHF maps** (when used with [i18n-gen](https://crates.io/crates/i18n-gen)).

---

## ✨ Features
- ✅ **Runtime JSON mode** — load translations directly from `locales/*.json`
- ⚡ **Compile-time PHF mode** — use generated Rust maps for O(1) lookups
- 🔄 **Fallback logic** — handles tags like `en-IN-BR → en-IN → en`
- 🔐 **Type-safe keys** — auto-generated `MessageKey` enum
- 🚀 **Zero-cost lookups** — powered by [`phf`](https://crates.io/crates/phf)

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
i18n-runtime = "0.1"

# Only required if you use generated PHF mode
phf = "0.11"
````

---

## 🚀 Quick Start

### Runtime JSON mode

1. Define `messages.schema.json`:

```json
{
  "keys": ["welcome", "login_success", "login_failed"]
}
```

2. Add a locale file `locales/en.json`:

```json
{
  "welcome": "Welcome!",
  "login_success": "You have logged in successfully.",
  "login_failed": "Login failed. Please try again."
}
```

3. Use it in code:

```rust
use i18n_runtime::{I18n, Locale};

fn main() -> anyhow::Result<()> {
    let i18n = I18n::from_json_dir("./locales", "en")?;

    let tags = ["en", "hi-IN", "fr"];
    for tag in &tags {
        let loc = Locale::new(tag);
        let msg = i18n.get_owned(&loc, "welcome").unwrap_or("<missing>".into());
        println!("{:<6} => {}", tag, msg);
    }

    Ok(())
}
```

Output:

```
en     => Welcome!
hi-IN  => स्वागत है!
fr     => Welcome!
```

---

### Generated PHF mode

1. Install [i18n-gen](https://crates.io/crates/i18n-gen):

```bash
cargo install i18n-gen
```

2. Run the generator:

```bash
i18n-gen ./ ./src/generated_i18n
```

This creates:

```
src/generated_i18n/generated_keys.rs   # MessageKey enum
src/generated_i18n/locales/EN.rs       # en.json → phf::Map
src/generated_i18n/locales/HI_IN.rs    # hi-IN.json → phf::Map
src/generated_i18n/mod.rs              # registry
```

3. Use in code:

```rust
// include generated files
include!("generated_i18n/generated_keys.rs");
mod generated_i18n { include!("generated_i18n/mod.rs"); }

use i18n_runtime::{I18n, Locale};

fn main() {
    let registry = generated_i18n::get_generated_registry();
    let i18n = I18n::from_generated_registry(registry, "en");

    let msg = i18n
        .get_by_str_key(&Locale::new("hi-IN"), MessageKey::Welcome.as_str())
        .unwrap();
    println!("{}", msg); // स्वागत है!
}
```

---

## 🔄 Fallback Logic

`en-IN-BR` → `en-IN` → `en` → fallback locale.

---

## 🧩 Ecosystem

* [**i18n-runtime**](https://crates.io/crates/i18n-runtime) — runtime API (this crate)
* [**i18n-gen**](https://crates.io/crates/i18n-gen) — generator CLI for compile-time PHF maps

---

## 📜 License

MIT

---
