use std::net::SocketAddr;

use axum::Router;
use proj_lua::{LUA_CTX, ROUTER};
use proj_macro::plugin_preset;
use proj_web::LuaRouterService;

#[tokio::main]
async fn main() {
    LUA_CTX
        .load(plugin_preset!("init.lua"))
        .exec()
        .expect("can't load file");
    let lua_service = LuaRouterService::new(ROUTER.clone());
    let app = Router::new().fallback_service(lua_service);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
