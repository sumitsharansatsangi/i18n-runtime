# 🌟 i18n-app (Demo)

This is a **demo application** that shows how to use  
[`i18n-runtime`](https://crates.io/crates/i18n-runtime) (the runtime API)  
together with [`i18n-gen`](https://crates.io/crates/i18n-gen) (the generator CLI).

---

## 📦 What’s inside

- `messages.schema.json` → defines the allowed translation keys
- `locales/` → JSON files with translations (`en.json`, `hi-IN.json`, …)
- `src/main.rs` → demo app showing both **runtime JSON mode** and **generated PHF mode**

---

## 🚀 Run the demo

Clone the workspace and from the root run:

```bash
# (A) Run using runtime JSON mode
cargo run -p i18n-app
````

You’ll see output like:

```
--- demo: runtime JSON mode ---
en     => Welcome!
hi-IN  => स्वागत है!
fr     => Welcome!

--- demo: generated mode ---
en-IN LoginSuccess => You have logged in successfully.
```

---

## ⚡ Try generated mode

1. Build the generator:

```bash
cargo build -p i18n-gen --release
```

2. Run it against this app:

```bash
target/release/i18n-gen ./i18n-app ./i18n-app/src/generated_i18n
```

3. Inspect generated code:

```
i18n-app/src/generated_i18n/generated_keys.rs
i18n-app/src/generated_i18n/locales/EN.rs
i18n-app/src/generated_i18n/locales/HI_IN.rs
i18n-app/src/generated_i18n/mod.rs
```

4. Re-run the demo:

```bash
cargo run -p i18n-app
```

Now the **Generated mode** section will use compile-time PHF maps and the type-safe `MessageKey` enum.

---

## 🔄 Workflow

* Add or update keys in `messages.schema.json`
* Update translations in `locales/*.json`
* Re-run `i18n-gen`
* Rebuild → app picks up new translations instantly

---

## 🧩 Related crates

* [**i18n-runtime**](https://crates.io/crates/i18n-runtime) — runtime i18n API
* [**i18n-gen**](https://crates.io/crates/i18n-gen) — generator CLI
* **i18n-app** (this crate) — demo application showing both modes

---

## 📜 License

MIT