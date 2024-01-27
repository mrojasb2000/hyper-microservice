use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let builder = Server::bind(&addr);
    let server =
        builder.serve(|| service_fn_ok(|_| Response::new(Body::from("Almost microservice..."))));
    let server = server.map_err(drop);
    hyper::rt::run(server);
}
