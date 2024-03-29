// PAWS - Protocol to Access White Space Database
// Compliant with RFC 7545

use paws::error::*;
use paws::method::*;
use paws::{Request, InitReq, RegistrationReq};
use paws::{Response, InitResp, RegistrationResp};
use paws::version::*;
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use warp::Filter;
use tracing::{debug, error, info};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::try_init().unwrap();
    let port = 3030;
    info!("Starting server on port {}", port);
    info!("API: localhost:{}/v1beta", port);

    //   The POST method is the only method REQUIRED for PAWS.
    //   If a Database chooses to support GET, it MUST be an escaped URI.
    //   The Database MAY refuse to support the GET request by returning an HTTP error code,
    //   such as 405 (method not allowed).

    //    PAWS APIs
    //    localhost:3030/v1beta

    let index = warp::get()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path::end())
        .and_then(home);

    // GET /paws/version
    // curl -GET localhost:3030/v1beta/paws/version
    let paws_version = warp::get()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path("version"))
        .and(warp::path::end())
        .and(warp::addr::remote())
        .and_then(version_handler);

    // A PAWS request message is carried in the body of an HTTP POST request
    // POST /paws/init
    let paws_init = warp::post()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path("init"))
        .and(warp::path::end())
        .and(warp::addr::remote())
        .and(json_body())
        .and_then(init_handler);

    // POST /paws/register
    // Check the README file for an example
    let paws_register = warp::post()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(warp::addr::remote())
        .and(json_body())
        .and_then(register_handler);

    let routes = paws_version.or(paws_init).or(index).or(paws_register);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

// Handlers
async fn version_handler(addr: Option<SocketAddr>) -> Result<impl warp::Reply, warp::Rejection> {
    let src_addr = addr
        .map(|socket_addr| socket_addr.ip().to_string())
        .unwrap_or_else(|| "Unknown".into());
    info!("msg=\"GET /paws/version\" src_ip={}", src_addr);
    let mut result = HashMap::new();
    result.insert(String::from("pawsVersion"), json!(PAWS_VERSION));
    Ok(warp::reply::json(&result))
}

async fn home() -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    result.insert(String::from("message"), json!("Welcome to the PAWS API"));
    result.insert(String::from("pawsVersion"), json!(PAWS_VERSION));
    Ok(warp::reply::json(&result))
}

async fn init_handler(
    addr: Option<SocketAddr>,
    req: Request<InitReq>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let src_addr = addr
        .map(|socket_addr| socket_addr.ip().to_string())
        .unwrap_or_else(|| "Unknown".into());
    if let Some(Method::Init) = req.method() {
        info!("msg=\"POST /paws/init\" src_ip={}", src_addr);
        let msg_type = req.mtype();
        if msg_type != "INIT_REQ".to_string() {
            let err = ErrorResponse::new(ErrorCode::InvalidValue);
            error!("msg=\"POST /paws/init\" err_msg=\"{:?} Expected INIT_REQ message but found {}\" src_ip={}",err.error(), msg_type, src_addr);
            return Ok(warp::reply::json(&err));
        }
        // Get ruleset from Request
        let ruleset = req.ruleset();
        let res = Response::new(ruleset);
        Ok(warp::reply::json(&res))
    } else {
        let err = ErrorResponse::new(ErrorCode::InvalidValue);
        error!("msg=\"POST /paws/init\" err_msg=\"{:?} Expected method:spectrum.paws.init but found {}\" src_ip={}",err.error(), req.method, src_addr);
        Ok(warp::reply::json(&err))
    }
}

async fn register_handler(
    addr: Option<SocketAddr>,
    req: Request<RegistrationReq>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let src_addr = addr
        .map(|socket_addr| socket_addr.ip().to_string())
        .unwrap_or_else(|| "Unknown".into());
    if let Some(Method::Register) = req.method() {
        info!("msg=\"POST /paws/register\" src_ip={}", src_addr);
        let msg_type = req.mtype();
        if msg_type != "REGISTRATION_REQ".to_string() {
            let err = ErrorResponse::new(ErrorCode::InvalidValue);
            error!("msg=\"POST /paws/register\" err_msg=\"{:?}: Expected REGISTRATION_REQ message type but found '{}'\" src_ip={}",err.error(), msg_type, src_addr);
            return Ok(warp::reply::json(&err));
        }
        //let res = Response::new();
        let mut n = HashMap::new();
        n.insert("test".to_string(), json!("register"));
        Ok(warp::reply::json(&n))
    } else {
        let err = ErrorResponse::new(ErrorCode::NotRegistered);
        error!(
            "msg=\"POST /paws/register\" err_msg=\"{:?}\" src_ip={}",
            err.error(),
            src_addr
        );
        Ok(warp::reply::json(&err))
    }
}

fn json_body() -> impl Filter<Extract = (Request,), Error = warp::Rejection> + Clone {
    warp::body::json()
}
