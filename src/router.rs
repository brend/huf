use hyper::{Body, Request, Response};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Router {
    routes: Arc<HashMap<String, Box<dyn Fn(Request<Body>) -> Response<Body> + Send + Sync>>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(HashMap::new()),
        }
    }

    pub fn get(&mut self, path: &str, handler: impl Fn(Request<Body>) -> Response<Body> + Send + Sync + 'static) {
        Arc::get_mut(&mut self.routes).unwrap().insert(path.to_string(), Box::new(handler));
    }

    pub fn handle(&self, req: Request<Body>) -> Response<Body> {
        if let Some(handler) = self.routes.get(req.uri().path()) {
            handler(req)
        } else {
            Response::new(Body::from("404 Not Found"))
        }
    }
}