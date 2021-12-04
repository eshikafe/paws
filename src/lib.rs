// (c) 2021 TVWS-Project
// PAWS - Protocol to Access White Space Database
// Compliant with RFC 7545

pub mod db;
pub mod error;
pub mod message;
pub mod parameters;
pub mod types;
pub mod version;

#[cfg(test)]
mod test_cases {
    use super::db;
    use super::error::*;
    use super::message::*;
    use super::parameters::get_mac_addr;
    #[test]
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
    #[ignore]
    fn test_message() {
        let req = Request::new(Method::Init);
        let s = serde_json::to_string_pretty(&req).unwrap();
        println!("JSON: PAWS INIT_REQ: {}", s);
        println!("Struct: PAWS INIT_REQ: {:?}", req);
        // println!("Struct: PAWS INIT_REQ: {:?}", init_req);
    }

    #[test]
    fn test_error() {
        let err = ErrorResponse::new(ErrorCode::OutsideCoverage);
        let s = serde_json::to_string_pretty(&err).unwrap();
        println!("PAWS Error: {}", s);
    }
}
