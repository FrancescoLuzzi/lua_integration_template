mod proj;

use anyhow::{Ok, Result};
use mlua::Lua;
pub use proj::router::{LuaRouter, Method, ROUTER};
use std::sync::LazyLock;

pub static LUA_CTX: LazyLock<Lua> = LazyLock::new(|| init_lua().expect("can't load lua env"));

fn init_lua() -> Result<Lua> {
    let lua = Lua::new();
    let globals = lua.globals();

    globals.raw_set("proj", proj::compose(&lua)?)?;
    Ok(lua)
}
