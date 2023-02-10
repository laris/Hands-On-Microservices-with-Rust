//extern crate futures;
//extern crate hyper;
//extern crate rand;
//#[macro_use]
//extern crate serde_derive;
//extern crate serde_json;

use std::ops::Range;
//use futures::{future, Future, Stream};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::{service_fn, make_service_fn};
use rand::Rng;
use rand::distributions::{Bernoulli, Normal, Uniform};
use serde_derive::{Serialize, Deserialize};
use log::{debug, info, trace, warn};
use dotenv::dotenv;
use futures::TryStreamExt as _;
use futures::stream::StreamExt as _;

static INDEX: &[u8] = b"Random Microservice";

#[derive(Deserialize)]
#[serde(tag = "distribution", content = "parameters", rename_all = "lowercase")]
enum RngRequest {
    Uniform {
        #[serde(flatten)]
        range: Range<i32>,
    },
    Normal {
        mean: f64,
        std_dev: f64,
    },
    Bernoulli {
        p: f64,
    },
}

#[derive(Serialize)]
struct RngResponse {
    value: f64,
}

fn handle_request(request: RngRequest) -> RngResponse {
    let mut rng = rand::thread_rng();
    let value = {
        match request {
            RngRequest::Uniform { range } => {
                rng.sample(Uniform::from(range)) as f64
            },
            RngRequest::Normal { mean, std_dev } => {
                rng.sample(Normal::new(mean, std_dev)) as f64
            },
            RngRequest::Bernoulli { p } => {
                rng.sample(Bernoulli::new(p)) as i8 as f64
            },
        }
    };
    RngResponse { value }
}

async fn microservice_handler(req: Request<Body>)
    -> Result<Response<Body>, Error>
{
    debug!("Request:\n{req:#?}");
    let response = {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") | (&Method::GET, "/random") => {
                //Box::new(future::ok(Response::new(INDEX.into())))
                Response::new(INDEX.into())
            },
            (&Method::POST, "/random") => {
                /*
                let body = req.into_body().concat()
                    .map(|chunks| {
                        let res = serde_json::from_slice::<RngRequest>(chunks.as_ref())
                            .map(handle_request)
                            .and_then(|resp| serde_json::to_string(&resp));
                        match res {
                            Ok(body) => {
                                Response::new(body.into())
                            },
                            Err(err) => {
                                Response::builder()
                                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                                    .body(err.to_string().into())
                                    .unwrap()
                            },
                        }
                    });
                Box::new(body)
                */
                // Await the full body to be concatenated into a single `Bytes`...
                let full_body = hyper::body::to_bytes(req.into_body()).await?;
                let resp = serde_json::from_slice::<RngRequest>(&full_body)
                    .map(handle_request)
                    .and_then(|resp| serde_json::to_string(&resp));
                match resp {
                    Ok(body) => {
                        Response::new(body.into())
                    },
                    Err(e) => {
                        Response::builder()
                            .status(StatusCode::UNPROCESSABLE_ENTITY)
                            .body(e.to_string().into())
                            .unwrap()
                    },
                }
            },
            _ => {
                let resp = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body("Not Found".into())
                    .unwrap();
                //Box::new(future::ok(resp))
                resp
            },
        }
    };
    debug!("Response\n{response:#?}");
    Ok(response)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    // init addr
    let addr = ([127, 0, 0, 1], 8080).into();
    debug!("Trying to bind server to address: {}", addr);
    // create service and handler
    let make_svc = make_service_fn(|_| async move {
        Ok::<_, Error>(service_fn(|req|microservice_handler(req)))
    });
    // create server
    let server = Server::bind(&addr).serve(make_svc);
    info!("Used address: {}", server.local_addr());
    debug!("Run!");
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    /*
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(|| {
        service_fn(microservice_handler)
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
    */
}
