use hyper::{Body, Response, Server, Error};
use hyper::service::{service_fn, make_service_fn};

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let make_svc = make_service_fn(|_| {
        let service = service_fn(|req| {
            println!("{:#?}", req);
            let random_byte = rand::random::<u8>();
            let resp = Response::new(Body::from(random_byte.to_string()));
            println!("{:#?}", resp);
            async move { Ok::<_, Error>(resp)}
        });
        async move { Ok::<_, Error>(service) }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
