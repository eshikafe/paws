use warp::Filter;
use serde_json::{json, Value};
use std::collections::HashMap;
use paws::version::*;
use paws::method::*;
use paws::message::*;
// use jsonrpc_v2::{Data, Error, Params, Server};
//use log::info;

#[tokio::main]
async fn main() {
    // env_logger::init();
    start();
}

pub async fn start() {

    let port = 3030;
    println!("PAWS server starting on port {}", port);

//   The POST method is the only method REQUIRED for PAWS.  
//   If a Database chooses to support GET, it MUST be an escaped URI.
//   The Database MAY refuse to support the GET request by returning an HTTP error code,
//   such as 405 (method not allowed).


//    PAWS APIs
//    localhost:3030/v1beta/spectrumdb

    // GET /version
    // curl -GET localhost:3030/v1beta/spectrumdb/version
    let get_version = warp::get()
        .and(warp::path("v1beta"))
        .and(warp::path("spectrumdb"))
        .and(warp::path("version"))
        .and(warp::path::end())
      //  .and(json_body())
        .and_then(get_paws_version);

     
//     // POST /init
//     // curl --request POST localhost:3030/v1beta/spectrumdb/init
//     let post_init = warp::post()
//         .and(warp::path("v1beta"))
//         .and(warp::path("spectrumdb"))
//         .and(warp::path("init"))
//         .and(warp::path::end())
//         .and(json_body())
//         .and_then(paws_init);

//     // POST /register
//     // curl --request POST localhost:3030/v1beta/spectrumdb/register
//     let post_register = warp::post()
//         .and(warp::path("v1beta"))
//         .and(warp::path("spectrumdb"))
//         .and(warp::path("register"))
//         .and(warp::path::end())
//         .and_then(paws_register);

    let routes = get_version; //.or(post_init).or(post_register);

    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

async fn get_paws_version() -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    result.insert(String::from("PAWS Version"), json!(PAWS_VERSION));
    Ok(warp::reply::json(&result))
}

// async fn paws_init(req: Request<InitReq>) -> Result<impl warp::Reply, warp::Rejection> {
//     if req.method == String::from("spectrum.paws.init") {
//         let res = Response::<InitResp>::new("init");
//         Ok(warp::reply::json(&res))
//     }else {
//         Ok(warp::reply::json("Error"))
//     }
    
// }

// async fn paws_register() -> Result<impl warp::Reply, warp::Rejection> {
//     let mut result = HashMap::new();
//     result.insert(String::from("Method"), json!("spectrum.paws.register"));
//     Ok(warp::reply::json(&result))
// }

fn json_body() -> impl Filter<Extract = (Request<InitReq>,), Error = warp::Rejection> + Clone {
    warp::body::json()
}