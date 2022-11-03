// PAWS - Protocol to Access White Space Database
// Compliant with RFC 7545

pub mod db;
pub mod error;
pub mod method;
pub mod parameters;
pub mod types;
pub mod version;

mod request;
mod response;

pub use request::{InitReq,RegistrationReq, Request};
pub use response::{InitResp, RegistrationResp, Response};


#[cfg(test)]
mod test_cases {
    use super::db;
    use super::error::*;
    use super::method::*;
    use super::request::*;
    // use super::response::*;
    use super::parameters::get_mac_addr;
    #[test]
    #[ignore]
    fn test_redis_db() {
        let mut redis = db::redis_connect();
        db::set_ruleset(&mut redis, "nccId", "12345");
        let r = db::get_ruleset(&mut redis, "nccId");
        assert_eq!("12345", r);
    }
    #[test]
    #[ignore]
    fn test_mac_addr() {
        let m = get_mac_addr();
        println!("{}", m);
    }

    // #[test]
    // fn test_server() {
    //     server::start();
    // }

    #[test]
    fn init_req() {
        let request = Request::new(Method::Init, InitReq::new());
        let s = serde_json::to_string_pretty(&request).unwrap();
        println!("INIT_REQ::JSON {}", s);
        println!("INIT_REQ::Struct {:?}", request);
    }

    #[test]
    fn registration_req() {
        let request = Request::new(Method::Register, RegistrationReq::new());
        let s = serde_json::to_string_pretty(&request).unwrap();
        println!("REGISTRATION_REQ::JSON {}", s);
        println!("REGISTRATION_REQ::Struct {:?}", request);
    }

    #[test]
    fn test_error() {
        let err = ErrorResponse::new(ErrorCode::OutsideCoverage);
        let s = serde_json::to_string_pretty(&err).unwrap();
        println!("PAWS Error: {}", s);
    }
}
