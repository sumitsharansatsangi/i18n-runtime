// demo_app/src/main.rs

// include generated enum (defines MessageKey)
include!("generated_i18n/generated_keys.rs");

// include generated registry module (defines get_generated_registry and the statics)
mod generated_i18n {
    include!("generated_i18n/mod.rs");
}

use i18n_runtime::{I18n, Locale};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- demo: runtime JSON mode ---");
    let i18n = I18n::from_json_dir("./locales", "en")?;
    let tags = ["en-IN", "hi-IN", "en-US", "fr"];
    for tag in &tags {
        let loc = Locale::new(*tag);
        let s = i18n.get_owned(&loc, "welcome").unwrap_or_else(|| "<missing>".into());
        println!("{:<6} => {}", tag, s);
    }

    println!("\n--- demo: generated mode ---");
    // If the generated files are present, use them
    let registry = generated_i18n::get_generated_registry();
    let ig = I18n::from_generated_registry(registry, "en");

    // MessageKey is in the top-level include
    let l = Locale::new("en-IN");
    let v = ig.get_by_str_key(&l, MessageKey::LoginSuccess.as_str()).unwrap_or("<missing>");
    println!("en-IN EnumCreated => {}", v);

    Ok(())
}
