use http::{response, Request, Response};
use serde_json::{from_slice, Value};
use std::{
    future::Future,
    pin::{pin, Pin},
    sync::{Arc, RwLock},
    task::{Context, Poll},
};
use tower_service::Service;

// Create new service to handle and route lua defined routes
//
// https://docs.rs/tower-http/latest/src/tower_http/services/fs/serve_dir/mod.rs.html#389
// https://docs.rs/tower-http/latest/src/tower_http/services/fs/serve_dir/mod.rs.html#306
use proj_lua::LuaRouter;

pub struct LuaRouterService {
    router: Arc<RwLock<LuaRouter>>,
}

impl Service<Request<Vec<u8>>> for LuaRouterService {
    type Response = Response<Vec<u8>>;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Vec<u8>>) -> Self::Future {
        let callback = async move {
            let func = self
                .router
                .read()
                .expect("can't lock read in router") // maybe return 500?
                .route(req.method(), req.uri().path())
                .unwrap(); // WARN: BADDDD, this should return 404 or forward the call over?
            response::Builder::new().body([1,2] // WHAT IS HAPPENING
        };
        Box::pin(callback)
    }
}
