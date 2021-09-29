// Copyright (c) 2021 TVWS-Project
// PAWS - Protocol to Access White Space Database
// Compliant with RFC 7545

use paws::errors::*;
use paws::message::*;
use paws::version::*;

use env_logger::{fmt::Color, Target, WriteStyle};
use log::{error, info, LevelFilter};
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;
use std::net::SocketAddr;
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .format(|buf, record| {
            let ts = buf.timestamp();
            let mut ls = buf.style();
            ls.set_color(Color::Green);
            let module = record.module_path().unwrap();
            writeln!(
                buf,
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
    info!("Starting PAWS server on port {}", port);

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
        .and(warp::addr::remote())
        .and_then(get_paws_version);

    // POST /init
    // Check the README file for an example
    let post_init_proc = warp::post()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path("init"))
        .and(warp::path::end())
        .and(warp::addr::remote())
        .and(json_body())
        .and_then(paws_init);

    // POST /register
    // Check the README file for an example
    let post_register = warp::post()
        .and(warp::path("v1beta"))
        .and(warp::path("paws"))
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(warp::addr::remote())
        .and(json_body())
        .and_then(paws_register);

    let routes = get_version.or(post_init_proc).or(index).or(post_register);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

async fn get_paws_version(addr: Option<SocketAddr>) -> Result<impl warp::Reply, warp::Rejection> {
    let src_addr = addr
                .map(|socket_addr| socket_addr.ip().to_string())
                .unwrap_or_else(|| "Unknown".into());
    info!("Received 'GET /version' request from {}",src_addr);
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
async fn paws_init(addr: Option<SocketAddr>, req: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let src_addr = addr
                .map(|socket_addr| socket_addr.ip().to_string())
                .unwrap_or_else(|| "Unknown".into());
    if let Some(Method::Init) = req.method() {
        info!("Received 'POST /init' request from {}", src_addr);
        let msg_type = req.mtype();
        if msg_type != "INIT_REQ".to_string() {
            let err = ErrorResponse::new(ErrorCode::InvalidValue);
            error!("Error in 'POST /init' request received from {} {:?}. Expected INIT_REQ as message type but found {}",src_addr, err.error(), msg_type);
            return Ok(warp::reply::json(&err));
        }
        // Get ruleset from Request
        let ruleset = req.ruleset();
        let res = Response::new(ruleset);
        Ok(warp::reply::json(&res))
    } else {
        let err = ErrorResponse::new(ErrorCode::InvalidValue);
        error!("Error in 'POST /init' request received from {} {:?}. Expected 'spectrum.paws.init' as method but found {}",src_addr, err.error(), req.method);
        Ok(warp::reply::json(&err))
    }
}

async fn paws_register(addr: Option<SocketAddr>, req: Request) -> Result<impl warp::Reply, warp::Rejection> {
    let src_addr = addr
                .map(|socket_addr| socket_addr.ip().to_string())
                .unwrap_or_else(|| "Unknown".into());
    if let Some(Method::Register) = req.method() {
        info!("Received 'POST /register' request received from {}", src_addr);
        let msg_type = req.mtype();
        if msg_type != "REGISTRATION_REQ".to_string() {
            let err = ErrorResponse::new(ErrorCode::InvalidValue);
            error!("Error in 'POST /init' request received from {} {:?}. Expected REGISTRATION_REQ as message type but found {}",src_addr, err.error(), msg_type);
            return Ok(warp::reply::json(&err));
        }
        //let res = Response::new();
        let mut n = HashMap::new();
        n.insert("test".to_string(), json!("register"));
        Ok(warp::reply::json(&n))
    } else {
        let err = ErrorResponse::new(ErrorCode::NotRegistered);
        error!("Error in 'POST /register' request received from {} {:?}",src_addr, err.error());
        Ok(warp::reply::json(&err))
    }
}

fn json_body() -> impl Filter<Extract = (Request,), Error = warp::Rejection> + Clone {
    warp::body::json()
}
