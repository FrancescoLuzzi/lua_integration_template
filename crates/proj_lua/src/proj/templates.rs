use minijinja::Environment;
use mlua::{Function, IntoLua as _, Lua, Table};
use std::sync::{LazyLock, RwLock};

static TEMPLATES: LazyLock<RwLock<Environment<'static>>> =
    LazyLock::new(|| RwLock::new(Environment::new()));

struct LuaTemplaterRegister {}

impl LuaTemplaterRegister {
    fn render_template(lua: &Lua) -> mlua::Result<Function> {
        lua.create_function(|_, (name, tbl_parms): (mlua::String, mlua::Table)| {
            TEMPLATES
                .read()
                .unwrap()
                .get_template(&name.to_str()?)
                .map_err(anyhow::Error::new)?
                .render(tbl_parms)
                .map_err(|err| mlua::Error::RuntimeError(err.to_string()))
        })
    }

    fn add_template(lua: &Lua) -> mlua::Result<Function> {
        lua.create_function(|_, (name, template): (mlua::String, mlua::String)| {
            TEMPLATES
                .write()
                .unwrap()
                .add_template_owned(name.to_string_lossy(), template.to_string_lossy())
                .map_err(anyhow::Error::new)?;
            Ok(())
        })
    }
}

pub fn register(lua: &Lua) -> mlua::Result<Table> {
    let index = lua.create_function(|lua, (ts, key): (Table, mlua::String)| {
        let value = match key.as_bytes().as_ref() {
            b"render_template" => LuaTemplaterRegister::render_template(lua)?,
            b"add_template" => LuaTemplaterRegister::add_template(lua)?,
            _ => return Ok(mlua::Value::Nil),
        }
        .into_lua(lua)?;

        ts.raw_set(key, value.clone())?;
        Ok(value)
    })?;

    let tbl = lua.create_table_with_capacity(0, 10)?;
    tbl.set_metatable(Some(lua.create_table_from([("__index", index)])?));

    Ok(tbl)
}
