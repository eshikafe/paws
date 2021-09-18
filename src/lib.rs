// (c) 2021 TVWS-Project
// PAWS - Protocol to Access White Space Database
// Compliant with RFC 7545

// pub mod errors;
pub mod types;
pub mod parameters;
pub mod method;
pub mod version;
pub mod redis_client;
pub mod server;
pub mod message;



#[cfg(test)]
mod test_cases {
    // use super::redis_client as rc;
    // use super::parameters::get_mac_addr;
    // use super::server;
    // use super::message;
    // use super::method;
    
    #[test]
    #[ignore]
    fn test_redis_db() {
        let r1 = rc::get_ruleset("nccId");
        let r2 = rc::get_ruleset("fccId");
        assert_eq!("123", r1);
        assert_eq!("12345", r2);
        println!("value for nccId = {}", r1);
        println!("value for fccId = {}", r2);
        
    }
    
    #[test]
    #[ignore]
    fn test_mac_addr() {
        let m = get_mac_addr();
        println!("{}",m);
    }

    
    #[test]
   
    fn test_server() {
        server::start();
    }

    #[test]
    #[ignore]
    fn test_message() {
        let init_req = message::InitReq::new();
        let req = message::Request::new(init_req);
        let s = serde_json::to_string_pretty(&req).unwrap();
        println!("JSON: PAWS INIT_REQ: {}", s);
        println!("Struct: PAWS INIT_REQ: {:?}", req);
        // println!("Struct: PAWS INIT_REQ: {:?}", init_req);
    }
}