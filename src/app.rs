use crate::router::Router;
use hyper::{Body, Request, Response};
use std::sync::Arc;

#[derive(Clone)]
pub struct App {
    router: Router,
    middlewares: Arc<Vec<Box<dyn Fn(Request<Body>) -> Request<Body> + Send + Sync>>>,
}

impl App {
    pub fn new(router: Router) -> Self {
        Self {
            router,
            middlewares: Arc::new(Vec::new()),
        }
    }

    pub fn use_middleware(&mut self, mw: impl Fn(Request<Body>) -> Request<Body> + Send + Sync + 'static) {
        Arc::get_mut(&mut self.middlewares).unwrap().push(Box::new(mw));
    }

    pub fn handle(&self, req: Request<Body>) -> Response<Body> {
        let mut req = req;
        for mw in &*self.middlewares {
            req = mw(req);
        }
        self.router.handle(req)
    }
}