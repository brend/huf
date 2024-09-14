mod app;
mod router;

use app::App;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use router::Router;
use std::convert::Infallible;
use tokio::main;

// Sample route handler for home page
fn home_handler(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("Welcome to the home page!"))
}

// Sample route handler for about page
fn about_handler(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("About us"))
}

// Sample middleware to log requests
fn logging_middleware(req: Request<Body>) -> Request<Body> {
    println!("Incoming request to: {}", req.uri().path());
    req
}

#[main]
async fn main() {
    // Create a router and add routes
    let mut router = Router::new();
    router.get("/", home_handler);
    router.get("/about", about_handler);

    // Create an app and add middleware
    let mut app = App::new(router);
    app.use_middleware(logging_middleware);

    // Set up hyper server with the app's handler
    let make_svc = make_service_fn(move |_conn| {
        let app = app.clone(); // Clone the app to share across threads
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let response = app.handle(req);
                async move { Ok::<_, Infallible>(response) }
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}