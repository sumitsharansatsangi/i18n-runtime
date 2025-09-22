// GENERATED — DO NOT EDIT

include!("locales/EN_IN.rs");
include!("locales/EN.rs");
include!("locales/HI_IN.rs");

pub static GENERATED_REGISTRY: &'static [(&'static str, &'static phf::Map<&'static str, &'static str>)] = &[
    ("en-IN", &EN_IN),
    ("en", &EN),
    ("hi-IN", &HI_IN),
];

pub fn get_generated_registry() -> &'static [(&'static str, &'static phf::Map<&'static str, &'static str>)] {
    GENERATED_REGISTRY
}
