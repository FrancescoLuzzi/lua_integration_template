#[macro_export]
macro_rules! plugin_preset {
    ($name:literal) => {{
        #[cfg(debug_assertions)]
        {
            std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/preset/", $name,)).expect(concat!(
                "failed to read '{{project-name}}_lua/preset/",
                $name,
            ))
        }
        #[cfg(not(debug_assertions))]
        {
            &include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/preset/", $name,))[..]
        }
    }};
}
