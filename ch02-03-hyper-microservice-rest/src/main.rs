use std::fmt;
use std::sync::{Arc, Mutex};
use slab::Slab;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
// curl -viA '' http://localhost:8080
// curl -viA '' -X POST http://localhost:8080

const INDEX: &str = r#"
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

type UserId = u64;

struct UserData;

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{}")
    }
}

type UserDb = Arc<Mutex<Slab<UserData>>>;

const USER_PATH: &str = "/user/";

async fn microservice_handler(req: Request<Body>, user_db: UserDb)
    -> Result<Response<Body>, Error>
{
    println!("Request:\n{:#?}", req);
    let response = {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                Response::new(INDEX.into())
            },
            (method, path) if path.starts_with(USER_PATH) => {
                let user_id = path.trim_start_matches(USER_PATH)
                    .parse::<UserId>()
                    .ok()
                    .map(|x| x as usize);
                println!("Request match method: {:?}, path: {:?}, user_id: {:?}", &method, &path, &user_id);
                let user_db = user_db.clone();
                let mut users = user_db.lock().unwrap();
                match (method, user_id) {
                    (&Method::GET, Some(id)) => {
                        if let Some(data) = users.get(id) {
                            Response::new(data.to_string().into())
                        } else {
                            response_with_code(StatusCode::NOT_FOUND)
                        }
                    },
                    (&Method::POST, None) => {
                        let id = users.insert(UserData);
                        Response::new(id.to_string().into())
                    },
                    (&Method::POST, Some(_)) => {
                        response_with_code(StatusCode::BAD_REQUEST)
                    },
                    (&Method::PUT, Some(id)) => {
                        if let Some(user) = users.get_mut(id) {
                            *user = UserData;
                            response_with_code(StatusCode::OK)
                        } else {
                            response_with_code(StatusCode::NOT_FOUND)
                        }
                    },
                    (&Method::DELETE, Some(id)) => {
                        if users.contains(id) {
                            users.remove(id);
                            response_with_code(StatusCode::OK)
                        } else {
                            response_with_code(StatusCode::NOT_FOUND)
                        }
                    },
                    _ => {
                        response_with_code(StatusCode::METHOD_NOT_ALLOWED)
                    },
                }
            },
            _ => {
                response_with_code(StatusCode::NOT_FOUND)
           },
        }
    };
    println!("Response:\n{:#?}", response);
    Ok(response)
}

fn response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let user_db = Arc::new(Mutex::new(Slab::new()));
    let make_svc = make_service_fn(move|_| {
        let user_db = user_db.clone();
        let service = service_fn(move |req| microservice_handler(req, user_db.clone()));
        async move { Ok::<_, Error>(service) }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
