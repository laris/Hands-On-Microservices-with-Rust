use hyper::{Body, Response, Server, Error};
use hyper::service::{make_service_fn, service_fn};
// curl -viA '' http://localhost:8080

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let make_service = make_service_fn(|_| async move {
        Ok::<_, Error>(service_fn(|req| async move {
            println!("{:#?}", req);
            let resp = Response::new(Body::from("Rust Microservice"));
            println!("{:#?}", resp);
            Ok::<_, Error>(resp)
        }))
    });
    let server = Server::bind(&addr).serve(make_service);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
