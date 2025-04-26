use http::{response, Request, Response};
use proj_lua::LuaParams;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock},
    task::{Context, Poll},
};
use tower_service::Service;

// Create new service to handle and route lua defined routes
//
// https://docs.rs/tower-http/latest/src/tower_http/services/fs/serve_dir/mod.rs.html#389
// https://docs.rs/tower-http/latest/src/tower_http/services/fs/serve_dir/mod.rs.html#306
use proj_lua::LuaRouter;

#[derive(Clone)]
pub struct LuaRouterService {
    router: Arc<RwLock<LuaRouter>>,
}

impl LuaRouterService {
    pub fn new(router: Arc<RwLock<LuaRouter>>) -> Self {
        Self { router }
    }
}

impl<ReqBody> Service<Request<ReqBody>> for LuaRouterService {
    type Response = Response<String>;

    type Error = Infallible;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Infallible>> + Send + Sync>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.router.try_read().is_ok() {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let guard = self.router.read().expect("can't lock read in router"); // maybe return 500?
        let matched = guard
            .route(&req.method().clone(), req.uri().path())
            .unwrap(); // WARN: BADDDD, this should return 404 or forward the call over?
        let (func, params) = (matched.value.clone(), LuaParams::new(matched.params));
        let callback = async move {
            let response = func.call::<mlua::Value>(params).unwrap(); // WARN: BADDDD, this should return 404 or forward the call over?
            let response = serde_json::to_string(&response).unwrap(); // WARN: BADDDD, this should return 404 or forward the call over?
            Ok(response::Builder::new().body(response).unwrap()) // WHAT IS HAPPENING
        };
        Box::pin(callback)
    }
}
