use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

const INDEX: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Rust Microservice</title>
    </head>
    <body>
        <h3>Rust Microservice</h3>
    </body>
</html>
"#;

async fn microservice_handler(req: Request<Body>) -> Result<Response<Body>, Error>
//    -> impl Future<Item=Response<Body>, Error=Error>
{
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            println!("Request:\n{:#?}", req);
            let resp = Response::new(INDEX.into());
            println!("Response:\n{:#?}", resp);
            Ok(resp)
        },
        _ => {
            println!("Request:\n{:#?}", req);
            let resp = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            println!("Response:\n{:#?}", resp);
            Ok(resp)
        },
    }
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let make_svc = make_service_fn(|_| async move {
        Ok::<_, Error>(service_fn(|req|microservice_handler(req)))
    });

    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
