# 🌍 i18n-runtime & i18n-gen

**Effortless, fast, and type-safe internationalization (i18n) for Rust apps.**
Powered by `phf` for zero-cost lookups.

---

## ✨ Features

* **Runtime JSON mode** → Load translations from `locales/*.json` at runtime.
* **Compile-time PHF mode** → Generate Rust code with `i18n-gen` for maximum speed and safety.
* **Fallback logic** → Handles tags like `en-IN-BR` → `en-IN` → `en`.
* **Type-safe keys** → Auto-generated `MessageKey` enum from `messages.schema.json`.
* **Blazing fast** → Lookup uses `phf::Map` → O(1), no allocations.

---

## 🛠️ Installation

In your project’s `Cargo.toml`:

```toml
[dependencies]
i18n-runtime = "0.1"

# Only if you use generated mode
phf = "0.11"
```

To install the generator tool:

```bash
cargo install i18n-gen
```

---

## 🚀 Quick Start

### 1. Define message keys

Create `messages.schema.json` at project root:

```json
{
  "keys": [
    "welcome",
    "login_success",
    "login_failed"
  ]
}
```

### 2. Add locales

Create `locales/en.json`:

```json
{
  "welcome": "Welcome!",
  "login_success": "You have logged in successfully.",
  "login_failed": "Login failed. Please try again."
}
```

And `locales/hi-IN.json`:

```json
{
  "welcome": "स्वागत है!",
  "login_success": "आप सफलतापूर्वक लॉग इन हो गए हैं।",
  "login_failed": "लॉगिन विफल। कृपया पुनः प्रयास करें।"
}
```

---

## 📦 Option A — Runtime JSON mode

Use translations directly from JSON files.

```rust
use i18n_runtime::{I18n, Locale};

fn main() -> anyhow::Result<()> {
    // Load from ./locales, fallback = "en"
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

✅ Great for development or dynamic updates.

---

## ⚡ Option B — Generated PHF mode

Pre-generate Rust code for **fast, type-safe, allocation-free lookups**.

### 1. Run the generator

```bash
i18n-gen ./ ./src/generated_i18n
```

This creates:

```
src/generated_i18n/generated_keys.rs   // MessageKey enum
src/generated_i18n/locales/EN.rs       // en.json → phf::Map
src/generated_i18n/locales/HI_IN.rs    // hi-IN.json → phf::Map
src/generated_i18n/mod.rs              // registry
```

### 2. Use in your app

```rust
// include generated files
include!("generated_i18n/generated_keys.rs");
mod generated_i18n {
    include!("generated_i18n/mod.rs");
}

use i18n_runtime::{I18n, Locale};

fn main() {
    // Use the generated registry
    let registry = generated_i18n::get_generated_registry();
    let i18n = I18n::from_generated_registry(registry, "en");

    let loc = Locale::new("hi-IN");
    let msg = i18n.get_by_str_key(&loc, MessageKey::Welcome.as_str()).unwrap();
    println!("{}", msg); // स्वागत है!
}
```

✅ Perfect for production builds: fast and safe.

---

## 🔄 Fallback Logic

Locales can have up to **3 tags** (e.g. `en-IN-BR`).
Lookup automatically falls back:

```
"en-IN-BR" → "en-IN" → "en"
```

If no translation found, falls back to your configured default.

---

## 🤔 FAQ

**Q: Do I need `i18n-gen`?**
A: Only if you want compile-time PHF mode. Runtime mode works without it.

**Q: Should I check generated files into git?**
A: Yes, recommended. Then you don’t need `i18n-gen` on every machine.

**Q: Why `phf`?**
A: It gives constant-time lookups and stores translations in a perfect hash table with zero runtime overhead.

---

## 🧩 Ecosystem

* **[`i18n-runtime`](https://crates.io/crates/i18n-runtime)** — the library (this crate)
* **[`i18n-gen`](https://crates.io/crates/i18n-gen)** — the generator CLI

---

## 📜 License

MIT

---
