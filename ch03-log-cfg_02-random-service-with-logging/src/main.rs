use hyper::{Body, Response, Server, Error};
use hyper::service::{make_service_fn, service_fn};
use log::{debug, info, trace};
/*
RUST_LOG=trace cargo run
RUST_LOG=random_service=trace,warn cargo run
RUST_LOG_STYLE=auto cargo run
*/
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Rand Microservice - v0.1.0");
    trace!("Starting...");
    let addr = ([127, 0, 0, 1], 8080).into();
    trace!("Creating service handler...");
    let make_svc = make_service_fn(|_| {
        let service = service_fn(|req| {
            trace!("Incoming request is: {:?}", req);
            //println!("{:#?}", req);
            let random_byte = rand::random::<u8>();
            debug!("Generated value is: {}", random_byte);
            let resp = Response::new(Body::from(random_byte.to_string()));
            trace!("Response is: {:?}", resp);
            //println!("{:#?}", resp);
            async move { Ok::<_, Error>(resp)}
        });
        async move { Ok::<_, Error>(service) }
    });

    debug!("Trying to bind server to address: {}", addr);
    let server = Server::bind(&addr).serve(make_svc);
    info!("Used address: {}", server.local_addr());
    debug!("Run!");
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
