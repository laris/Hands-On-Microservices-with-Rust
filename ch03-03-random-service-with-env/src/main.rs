use std::env;
use dotenv::dotenv;
use hyper::{Body, Response, Server, Error};
use hyper::service::{make_service_fn, service_fn};
use log::{debug, info, trace};

#[tokio::main]
async fn main() {
    dotenv().ok();
    //pretty_env_logger::init();
    info!("Rand Microservice - v0.1.0");
    trace!("Starting...");
    //let addr = ([127, 0, 0, 1], 8080).into();
    // replace static from reading env
    let addr = env::var("ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1:8080".into())
                .parse()
                .expect("Can't parse ADDRESS variable");
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


/*
fn main() {
    dotenv().ok();
    env_logger::init();
    info!("Rand Microservice - v0.1.0");
    trace!("Starting...");
    let addr = env::var("ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".into())
        .parse()
        .expect("can't parse ADDRESS variable");
    debug!("Trying to bind server to address: {}", addr);
    let builder = Server::bind(&addr);
    trace!("Creating service handler...");
    let server = builder.serve(|| {
        service_fn_ok(|req| {
            trace!("Incoming request is: {:?}", req);
            let random_byte = rand::random::<u8>();
            debug!("Generated value is: {}", random_byte);
            Response::new(Body::from(random_byte.to_string()))
        })
    });
    info!("Used address: {}", server.local_addr());
    let server = server.map_err(drop);
    debug!("Run!");
    hyper::rt::run(server);
}
*/
