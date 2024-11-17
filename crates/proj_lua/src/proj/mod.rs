pub mod router;
pub mod templates;
use mlua::IntoLua as _;

pub fn compose(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let index = lua.create_function(move |lua, (ts, key): (mlua::Table, mlua::String)| {
        let value = match key.as_bytes().as_ref() {
            // App
            b"router" => router::register(lua)?,
            b"templates" => templates::register(lua)?,
            _ => return Ok(mlua::Value::Nil),
        }
        .into_lua(lua)?;

        ts.raw_set(key, value.clone())?;
        Ok(value)
    })?;

    let proj = lua.create_table_with_capacity(0, 40)?;
    proj.set_metatable(Some(lua.create_table_from([("__index", index)])?));

    Ok(proj)
}
