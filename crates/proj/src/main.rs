use proj_lua::{HttpMethod, LUA_CTX, ROUTER};
use proj_macro::plugin_preset;
fn main() {
    LUA_CTX
        .load(plugin_preset!("init.lua"))
        .exec()
        .expect("addio");
    println!(
        "{:?}",
        ROUTER
            .read()
            .unwrap()
            .route(HttpMethod::Get, "test")
            .unwrap()
            .call::<String>(())
    );
}
