// i18n-gen/src/main.rs
use anyhow::Context;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct Schema {
    keys: Vec<String>,
}

fn normalize_tag_for_build(tag: &str) -> anyhow::Result<String> {
    let parts: Vec<&str> = tag.split('-').filter(|p| !p.is_empty()).collect();
    if parts.is_empty() {
        return Ok(String::new());
    }
    if parts.len() > 3 {
        anyhow::bail!("Locale '{}' has more than 3 subtags ({}).", tag, parts.len());
    }
    let mut out = Vec::new();
    for (i, p) in parts.iter().enumerate() {
        if i == 0 {
            out.push(p.to_lowercase());
        } else {
            out.push(p.to_uppercase());
        }
    }
    Ok(out.join("-"))
}

fn ident_from_tag(tag: &str) -> String {
    tag.replace('-', "_").to_uppercase()
}

fn snake_to_camel(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(first) => first.to_ascii_uppercase().to_string() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

fn escape_rust_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\"', "\\\"")
}

fn main() -> anyhow::Result<()> {
    // usage: i18n-gen <source_root> <dest_dir>
    // defaults: src = current dir, dest = ./src/generated_i18n
    let args: Vec<String> = std::env::args().collect();
    let src = args.get(1).map(|s| PathBuf::from(s)).unwrap_or(std::env::current_dir()?);
    let dest = args.get(2).map(|s| PathBuf::from(s)).unwrap_or_else(|| src.join("src/generated_i18n"));

    println!("source root: {}", src.display());
    println!("dest dir: {}", dest.display());

    let schema_path = src.join("messages.schema.json");
    let locales_dir = src.join("locales");

    anyhow::ensure!(schema_path.exists(), "messages.schema.json not found at {}", schema_path.display());
    anyhow::ensure!(locales_dir.exists(), "locales/ dir not found at {}", locales_dir.display());

    let schema_str = fs::read_to_string(&schema_path)
        .with_context(|| format!("reading schema {}", schema_path.display()))?;
    let schema: Schema = serde_json::from_str(&schema_str)
        .context("parsing messages.schema.json")?;

    let key_set: std::collections::HashSet<String> = schema.keys.iter().cloned().collect();

    fs::create_dir_all(&dest)?;
    fs::create_dir_all(&dest.join("locales"))?;

    // write generated_keys.rs
    let gen_keys_path = dest.join("generated_keys.rs");
    {
        let mut out = String::new();
        out.push_str("// GENERATED — DO NOT EDIT\n\n");
        out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
        out.push_str("pub enum MessageKey {\n");
        for (i, key) in schema.keys.iter().enumerate() {
            out.push_str(&format!("    {}, // id = {}\n", snake_to_camel(key), i));
        }
        out.push_str("}\n\n");

        out.push_str("impl MessageKey {\n");
        out.push_str("    pub fn as_u32(self) -> u32 {\n        match self {\n");
        for (i, key) in schema.keys.iter().enumerate() {
            out.push_str(&format!("            MessageKey::{} => {},\n", snake_to_camel(key), i));
        }
        out.push_str("        }\n    }\n\n");

        out.push_str("    pub fn as_str(self) -> &'static str {\n        match self {\n");
        for key in schema.keys.iter() {
            out.push_str(&format!("            MessageKey::{} => \"{}\",\n", snake_to_camel(key), key));
        }
        out.push_str("        }\n    }\n\n");

        out.push_str("    pub fn from_u32(v: u32) -> Option<MessageKey> {\n        match v {\n");
        for (i, key) in schema.keys.iter().enumerate() {
            out.push_str(&format!("            {} => Some(MessageKey::{}),\n", i, snake_to_camel(key)));
        }
        out.push_str("            _ => None,\n        }\n    }\n}\n");
        fs::write(&gen_keys_path, out).with_context(|| format!("writing {}", gen_keys_path.display()))?;
        println!("wrote {}", gen_keys_path.display());
    }

    // collect tags to generate registry
    let mut tags: Vec<String> = Vec::new();

    // process locale files
    for entry in WalkDir::new(&locales_dir).min_depth(1).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let raw = path.file_stem().unwrap().to_string_lossy().to_string();
        let canonical = normalize_tag_for_build(&raw)?;
        if canonical.is_empty() {
            continue;
        }
        if tags.contains(&canonical) {
            anyhow::bail!("duplicate canonical tag {}", canonical);
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("reading locale file {}", path.display()))?;
        let json: Value = serde_json::from_str(&content)
            .with_context(|| format!("parsing json {}", path.display()))?;
        let obj = json.as_object().context("locale file not an object")?;

        // missing/extra
        let missing: Vec<_> = schema.keys.iter().filter(|k| !obj.contains_key(*k)).cloned().collect();
        if !missing.is_empty() {
            anyhow::bail!("{} missing keys {:?}", path.display(), missing);
        }
        let extra: Vec<_> = obj.keys().filter(|k| !key_set.contains(k.as_str())).cloned().collect();
        if !extra.is_empty() {
            anyhow::bail!("{} extra keys {:?}", path.display(), extra);
        }

        // build phf map source
        let mut map_src = String::new();
        for key in schema.keys.iter() {
            let v = obj.get(key).unwrap().as_str().context("value not string")?;
            map_src.push_str(&format!("    \"{}\" => \"{}\",\n", key, escape_rust_string(v)));
        }

        let ident = ident_from_tag(&canonical);
        let locale_file = dest.join("locales").join(format!("{}.rs", ident));
        let mut out = String::new();
        out.push_str("// GENERATED — DO NOT EDIT\n");
        // Avoid emitting local `use` imports that clash when included together.
        out.push_str(&format!("pub static {ident}: phf::Map<&'static str, &'static str> = phf::phf_map! {{\n", ident = ident));
        out.push_str(&map_src);
        out.push_str("};\n");
        fs::write(&locale_file, out).with_context(|| format!("writing {}", locale_file.display()))?;
        println!("wrote {}", locale_file.display());

        tags.push(canonical);
    }

    // generate mod.rs (registry) with a 'static' slice
    let mod_path = dest.join("mod.rs");
    {
        let mut out = String::new();
        out.push_str("// GENERATED — DO NOT EDIT\n\n");
        // include locale files (they declare statics with fully-qualified phf::Map)
        for tag in tags.iter() {
            let ident = ident_from_tag(tag);
            out.push_str(&format!("include!(\"locales/{ident}.rs\");\n", ident = ident));
        }
        out.push_str("\n");
        // generate a static slice so the registry has 'static lifetime
        out.push_str("pub static GENERATED_REGISTRY: &'static [(&'static str, &'static phf::Map<&'static str, &'static str>)] = &[\n");
        for tag in tags.iter() {
            let ident = ident_from_tag(tag);
            out.push_str(&format!("    (\"{tag}\", &{ident}),\n", tag = tag, ident = ident));
        }
        out.push_str("];\n\n");
        out.push_str("pub fn get_generated_registry() -> &'static [(&'static str, &'static phf::Map<&'static str, &'static str>)] {\n");
        out.push_str("    GENERATED_REGISTRY\n}\n");
        fs::write(&mod_path, out).with_context(|| format!("writing {}", mod_path.display()))?;
        println!("wrote {}", mod_path.display());
    }

    println!("codegen finished. Generated files in {}", dest.display());
    Ok(())
}
