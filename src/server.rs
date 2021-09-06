use warp::Filter;
use serde_json::{json, Value};
use std::collections::HashMap;
use crate::version::*;
use crate::methods::*;

#[tokio::main]
pub async fn start() {
    
    // PAWS APIs

    // GET /api/spectrumdb/v1beta/version
    // curl -GET localhost:3030/api/spectrumdb/v1beta/version
    let get_version = warp::get()
        .and(warp::path("api"))
        .and(warp::path("spectrumdb"))
        .and(warp::path("v1beta"))
        .and(warp::path("version"))
        .and(warp::path::end())
        .and_then(get_paws_version);

     
    // POST /api/spectrumdb/v1beta/init
    // curl --request POST localhost:3030/api/spectrumdb/v1beta/init
    let post_init = warp::post()
        .and(warp::path("api"))
        .and(warp::path("spectrumdb"))
        .and(warp::path("v1beta"))
        .and(warp::path("init"))
        .and(warp::path::end())
        .and(json_body())
        .and_then(paws_init);

    // POST /api/spectrumdb/v1beta/register
    // curl --request POST localhost:3030/api/spectrumdb/v1beta/register
    let post_register = warp::post()
        .and(warp::path("api"))
        .and(warp::path("spectrumdb"))
        .and(warp::path("v1beta"))
        .and(warp::path("register"))
        .and(warp::path::end())
        .and_then(paws_register);

    let routes = get_version.or(post_init).or(post_register);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn get_paws_version() -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    result.insert(String::from("PAWS Version"), json!(PAWS_VERSION));
    Ok(warp::reply::json(&result))
}

async fn paws_init(req: Request<InitReq>) -> Result<impl warp::Reply, warp::Rejection> {
    if req.method == String::from("spectrum.paws.init") {
        let res = Response::<InitResp>::new("init");
        Ok(warp::reply::json(&res))
    }else {
        Ok(warp::reply::json("Error"))
    }
    
}

async fn paws_register() -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    result.insert(String::from("Method"), json!("spectrum.paws.register"));
    Ok(warp::reply::json(&result))
}

fn json_body() -> impl Filter<Extract = (Request<InitReq>,), Error = warp::Rejection> {
    warp::body::json()
}