use matchit::Router;
use mlua::{Function, IntoLua, Lua, Table, UserData, Value};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

pub static ROUTER: LazyLock<Arc<RwLock<LuaRouter>>> = LazyLock::new(Arc::default);

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum HttpMethod {
    Get,
}

#[derive(Default, Clone)]
pub struct LuaRouter {
    router: HashMap<HttpMethod, Router<Function>>,
}

impl LuaRouter {
    fn get(&mut self, url: &str, lua_fn: Function) {
        self.router
            .entry(HttpMethod::Get)
            .or_default()
            .insert(url, lua_fn)
            .expect("error inserting new route");
    }
    pub fn route(&self, method: HttpMethod, url: &str) -> Option<&Function> {
        self.router.get(&method)?.at(url).ok().map(|x| x.value)
    }
}

struct LuaRouterRegister {}

impl LuaRouterRegister {
    fn get(lua: &Lua) -> mlua::Result<Function> {
        lua.create_function(|_, (url, method): (mlua::String, Function)| {
            ROUTER.write().unwrap().get(&url.to_string_lossy(), method);
            Ok(())
        })
    }
}
fn register_router(lua: &Lua) -> mlua::Result<Table> {
    let index = lua.create_function(|lua, (ts, key): (Table, mlua::String)| {
        let value = match key.as_bytes().as_ref() {
            b"get" => LuaRouterRegister::get(lua)?,
            _ => return Ok(Value::Nil),
        }
        .into_lua(lua)?;

        ts.raw_set(key, value.clone())?;
        Ok(value)
    })?;

    let ps = lua.create_table_with_capacity(0, 10)?;
    ps.set_metatable(Some(lua.create_table_from([("__index", index)])?));

    Ok(ps)
}

pub fn compose(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let index = lua.create_function(move |lua, (ts, key): (mlua::Table, mlua::String)| {
        let value = match key.as_bytes().as_ref() {
            // App
            b"router" => register_router(lua)?,
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
