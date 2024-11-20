use proj_lua::{Method, LUA_CTX, ROUTER};
use proj_macro::plugin_preset;
use std::thread;

fn hello_route() {
    let router = ROUTER.read().unwrap();
    let matched = router
        .route(&Method::GET, "hello/testo/testo")
        .expect("route not found");
    let callback = matched.value;
    let params = matched.params.iter().fold(
        LUA_CTX.create_table().expect("can't create table"),
        |tbl, (key, value)| {
            tbl.raw_set(key, value).expect("can't set item");
            tbl
        },
    );

    println!("{:?}", callback.call::<String>(params));
}

fn main() {
    LUA_CTX
        .load(plugin_preset!("init.lua"))
        .exec()
        .expect("can't load file");
    thread::spawn(hello_route).join().unwrap();
}
