// Copyright (c) 2021 TVWS-Project
// PAWS - Protocol to Access White Space Database
// Compliant with RFC 7545

use paws::version::*;
// use paws::method::*;
use paws::message::*;
use paws::errors::*;

use warp::Filter;
use serde_json::{json, Value};
use std::collections::HashMap;

// use jsonrpc_v2::{Data, Error, Params, Server};
// use log::info;

#[tokio::main]
async fn main() {

    // env_logger::init();
    let port = 3030;
    println!("Starting PAWS server on port {}", port);

//   The POST method is the only method REQUIRED for PAWS.  
//   If a Database chooses to support GET, it MUST be an escaped URI.
//   The Database MAY refuse to support the GET request by returning an HTTP error code,
//   such as 405 (method not allowed).


//    PAWS APIs
//    localhost:3030/v1beta/paws

    // GET /version
    // curl -GET localhost:3030/v1beta/paws/version
    let get_version = warp::get()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path("version"))
        .and(warp::path::end())
      //  .and(json_body())
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

    let routes = get_version.or(post_init_proc);//.or(post_register);

    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

async fn get_paws_version() -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    result.insert(String::from("pawsVersion"), json!(PAWS_VERSION));
    Ok(warp::reply::json(&result))
}

async fn paws_init(req: Request) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", req);
    if req.method == String::from("spectrum.paws.init") {

        // TODO: Ensure that the location is not outside the regulatory domain
        //  Use reverse geocoding 
        // if reverse_geocode(req.lat, req.long) == "ng" proceed otherwise return ErrorCode::OutsideCovergae

        // Get ruleset from Request
        let ruleset = req.ruleset();
        let res = Response::new(ruleset);
        Ok(warp::reply::json(&res))

    }else {
        let err = ErrorResponse::new(ErrorCode::Unsupported);
        Ok(warp::reply::json(&err))
    }
    
}

// TODO
// latitude = Request.location.loc.point.center.latitude
// longitude = Request.location.loc.point.center.longitude
// If the location is outside all regulatory domain supported by the
// Database, the Database MUST respond with an OUTSIDE_COVERAGE error
// Use reserve geocoding: convert coordinates to country
// Reference: https://www.geeksforgeeks.org/how-to-check-if-a-given-point-lies-inside-a-polygon

fn reverse_geocode(latitude: f64, longitude: f64) -> String{
    return String::from("ng");
}

// async fn paws_register() -> Result<impl warp::Reply, warp::Rejection> {
//     let mut result = HashMap::new();
//     result.insert(String::from("Method"), json!("spectrum.paws.register"));
//     Ok(warp::reply::json(&result))
// }

fn json_body() -> impl Filter<Extract = (Request,), Error = warp::Rejection> + Clone {
    warp::body::json()
}