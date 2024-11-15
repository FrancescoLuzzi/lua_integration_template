mod router;

use anyhow::{Ok, Result};
use mlua::Lua;
pub use router::{HttpMethod, ROUTER};
use std::sync::LazyLock;

pub static LUA_CTX: LazyLock<Lua> = LazyLock::new(|| init_lua().unwrap());

fn init_lua() -> Result<Lua> {
    let lua = Lua::new();
    let globals = lua.globals();

    globals.raw_set("proj", router::compose(&lua)?)?;
    Ok(lua)
}
