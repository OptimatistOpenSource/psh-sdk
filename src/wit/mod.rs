#![allow(clippy::missing_safety_doc)]
wit_bindgen::generate!({
    world: "bindings",
    path: "wit",
    generate_all,
});
