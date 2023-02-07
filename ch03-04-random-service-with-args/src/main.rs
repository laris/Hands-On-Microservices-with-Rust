use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, App};
use dotenv::dotenv;
use hyper::{Body, Response, Server, Error};
use hyper::service::{make_service_fn, service_fn};
use log::{debug, info, trace};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    // clap
    info!("Fix me!\nPrint info log before clap help!\nWe should add the get_matches method before any logging call because it also prints help messages. \nWe should avoid printing logs with the help description.");
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true))
        .arg(Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("ADDRESS")
                .help("Sets an address")
                .takes_value(true))
        .get_matches();
    // subcommands demo 
    /*
    let matches = App::new("Server with keys")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("run")
            .about("run the server")
            .arg(Arg::with_name("address")
                .short("a")
                .long("address")
                .takes_value(true)
                .help("address of the server"))
        .subcommand(SubCommand::with_name("key")
            .about("Generates a secret key for cookies")
        .get_matches();
    */
    info!("Rand Microservice - v0.1.0");
    trace!("Starting...");
    //let addr = ([127, 0, 0, 1], 8080).into();
    // replace static from reading env
    /*
    let addr = env::var("ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1:8080".into())
                .parse()
                .expect("Can't parse ADDRESS variable");
    */
    // now parse addr from clap
    let addr = matches.value_of("address")
        .map(|s| s.to_owned())
        .or(env::var("ADDRESS").ok())
        .unwrap_or_else(|| "127.0.0.1:8080".into())
        .parse()
        .expect("can't parse ADDRESS variable");
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
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("address")
             .short("a")
             .long("address")
             .value_name("ADDRESS")
             .help("Sets an address")
             .takes_value(true))
        .get_matches();
    info!("Rand Microservice - v0.1.0");
    trace!("Starting...");
    let addr = matches.value_of("address")
        .map(|s| s.to_owned())
        .or(env::var("ADDRESS").ok())
        .unwrap_or_else(|| "127.0.0.1:8080".into())
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
