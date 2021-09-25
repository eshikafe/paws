// Copyright (c) 2021 TVWS-Project
// PAWS - Protocol to Access White Space Database
// Compliant with RFC 7545

use paws::version::*;
use paws::message::*;
use paws::errors::*;

use warp::Filter;
use serde_json::{json, Value};
use std::collections::HashMap;
use log::{info, error, LevelFilter};
use env_logger::{WriteStyle, Target, fmt::Color} ;
use std::io::Write;

#[tokio::main]
async fn main() {
    env_logger::builder()
    .format(|buf, record| {
        let ts = buf.timestamp();
        let mut ls = buf.style();
        ls.set_color(Color::Green);
        let module = record.module_path().unwrap();
        writeln!(buf,
            "{} [{}] [{}] {}", 
            ts,
            ls.value(record.level()),
            module,
            record.args()
        )
    })
    .filter(None, LevelFilter::Info)
    .write_style(WriteStyle::Auto)
    .target(Target::Stdout)
    .init();
    let port = 3030;
    info!("Starting PAWS server on port {}", port );

//   The POST method is the only method REQUIRED for PAWS.  
//   If a Database chooses to support GET, it MUST be an escaped URI.
//   The Database MAY refuse to support the GET request by returning an HTTP error code,
//   such as 405 (method not allowed).


//    PAWS APIs
//    localhost:3030/v1beta/paws

    let index = warp::get()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path::end())
        .and_then(home);

    // GET /version
    // curl -GET localhost:3030/v1beta/paws/version
    let get_version = warp::get()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path("version"))
        .and(warp::path::end())
        .and_then(get_paws_version);

     
    // POST /init
    // Check the README file for an example
    let post_init_proc = warp::post()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path("init"))
        .and(warp::path::end())
        .and(json_body())
        .and_then(paws_init);

//     // POST /register
//     // Check the README file for an example
//     let post_register = warp::post()
//         .and(warp::path("v1beta"))
//         .and(warp::path("paws"))
//         .and(warp::path("register"))
//         .and(warp::path::end())
//         .and_then(paws_register);

    let routes = get_version.or(post_init_proc).or(index);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;

    //global::shutdown_tracer_provider();
}

async fn get_paws_version() -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received 'GET /version' request" );
    let mut result = HashMap::new();
    result.insert(String::from("pawsVersion"), json!(PAWS_VERSION));
    Ok(warp::reply::json(&result))
}

async fn home() -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    result.insert(String::from("message"), json!("Welcome to the PAWS API"));
    Ok(warp::reply::json(&result))
}

//  A PAWS request message is carried in the body of an HTTP POST request
async fn paws_init(req: Request) -> Result<impl warp::Reply, warp::Rejection> {
    // println!("{:?}", req);
    if req.method == String::from("spectrum.paws.init") {
        info!("Received 'POST /init' request." );
        // Get device location and apply reverse geocoding 
        let (lat, lon) = req.location();
        println!("Latitude: {}, Longitude: {}", lat, lon);

        // Get ruleset from Request
        let ruleset = req.ruleset();
        let res = Response::new(ruleset);
        Ok(warp::reply::json(&res))

    }else {
        let err = ErrorResponse::new(ErrorCode::Unimplemented);
        error!("Error in 'POST /init' request: {:?}", err.error());
        Ok(warp::reply::json(&err))
    }
    
}


// async fn paws_register() -> Result<impl warp::Reply, warp::Rejection> {
//     let mut result = HashMap::new();
//     result.insert(String::from("Method"), json!("spectrum.paws.register"));
//     Ok(warp::reply::json(&result))
// }

fn json_body() -> impl Filter<Extract = (Request,), Error = warp::Rejection> + Clone {
    warp::body::json()
}
