use matchit::Router;
use mlua::{Function, IntoLua as _, Lua, Table, Value};
use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

// TODO: make LuaRouter thread safe (RWLock per HttpMethod)
pub static ROUTER: LazyLock<RwLock<LuaRouter>> = LazyLock::new(Default::default);

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Head,
    Delete,
}

#[derive(Default, Clone)]
pub struct LuaRouter {
    router: HashMap<HttpMethod, Router<Function>>,
}

impl LuaRouter {
    #[inline]
    fn register_handler<T: AsRef<str>>(
        &mut self,
        method: HttpMethod,
        url: T,
        lua_fn: Function,
    ) -> anyhow::Result<()> {
        Ok(self
            .router
            .entry(method)
            .or_default()
            .insert(url.as_ref(), lua_fn)?)
    }

    fn get<T: AsRef<str>>(&mut self, url: T, lua_fn: Function) -> anyhow::Result<()> {
        self.register_handler(HttpMethod::Get, url, lua_fn)
    }

    fn post<T: AsRef<str>>(&mut self, url: T, lua_fn: Function) -> anyhow::Result<()> {
        self.register_handler(HttpMethod::Post, url, lua_fn)
    }

    fn put<T: AsRef<str>>(&mut self, url: T, lua_fn: Function) -> anyhow::Result<()> {
        self.register_handler(HttpMethod::Put, url, lua_fn)
    }

    fn delete<T: AsRef<str>>(&mut self, url: T, lua_fn: Function) -> anyhow::Result<()> {
        self.register_handler(HttpMethod::Delete, url, lua_fn)
    }

    fn patch<T: AsRef<str>>(&mut self, url: T, lua_fn: Function) -> anyhow::Result<()> {
        self.register_handler(HttpMethod::Patch, url, lua_fn)
    }

    fn head<T: AsRef<str>>(&mut self, url: T, lua_fn: Function) -> anyhow::Result<()> {
        self.register_handler(HttpMethod::Head, url, lua_fn)
    }

    pub fn route<'a>(
        &'a self,
        method: HttpMethod,
        url: &'a str,
    ) -> Option<matchit::Match<'a, 'a, &Function>> {
        self.router.get(&method)?.at(url).ok()
    }
}

struct LuaRouterRegister {}

impl LuaRouterRegister {
    fn get(lua: &Lua) -> mlua::Result<Function> {
        lua.create_function(|_, (url, lua_fn): (mlua::String, Function)| {
            Ok(ROUTER.write().unwrap().get(url.to_str()?, lua_fn)?)
        })
    }
    fn post(lua: &Lua) -> mlua::Result<Function> {
        lua.create_function(|_, (url, lua_fn): (mlua::String, Function)| {
            Ok(ROUTER.write().unwrap().post(url.to_str()?, lua_fn)?)
        })
    }
    fn put(lua: &Lua) -> mlua::Result<Function> {
        lua.create_function(|_, (url, lua_fn): (mlua::String, Function)| {
            Ok(ROUTER.write().unwrap().put(url.to_str()?, lua_fn)?)
        })
    }
    fn delete(lua: &Lua) -> mlua::Result<Function> {
        lua.create_function(|_, (url, lua_fn): (mlua::String, Function)| {
            Ok(ROUTER.write().unwrap().delete(url.to_str()?, lua_fn)?)
        })
    }
    fn patch(lua: &Lua) -> mlua::Result<Function> {
        lua.create_function(|_, (url, lua_fn): (mlua::String, Function)| {
            Ok(ROUTER.write().unwrap().patch(url.to_str()?, lua_fn)?)
        })
    }
    fn head(lua: &Lua) -> mlua::Result<Function> {
        lua.create_function(|_, (url, lua_fn): (mlua::String, Function)| {
            Ok(ROUTER.write().unwrap().head(url.to_str()?, lua_fn)?)
        })
    }
}

pub fn register(lua: &Lua) -> mlua::Result<Table> {
    let index = lua.create_function(|lua, (ts, key): (Table, mlua::String)| {
        let value = match key.as_bytes().as_ref() {
            b"get" => LuaRouterRegister::get(lua)?,
            b"post" => LuaRouterRegister::post(lua)?,
            b"put" => LuaRouterRegister::put(lua)?,
            b"delete" => LuaRouterRegister::delete(lua)?,
            b"patch" => LuaRouterRegister::patch(lua)?,
            b"head" => LuaRouterRegister::head(lua)?,
            _ => return Ok(Value::Nil),
        }
        .into_lua(lua)?;

        ts.raw_set(key, value.clone())?;
        Ok(value)
    })?;

    let tbl = lua.create_table_with_capacity(0, 10)?;
    tbl.set_metatable(Some(lua.create_table_from([("__index", index)])?));

    Ok(tbl)
}
