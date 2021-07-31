// (c) 2021 TVWS-Project
// PAWS - Protocol to Access White Space Database
// Compliant with RFC 7545

pub mod types;
pub mod parameters;
// pub mod methods;
// pub mod errors;
// pub mod version;
pub mod redis_client;
pub mod server;


#[cfg(test)]
mod paws_tests {
    use super::redis_client as rc;
    use super::parameters::get_mac_addr;
    
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
    fn test_mac_addr() {
        let m = get_mac_addr();
        println!("{}",m);
    }

    use super::server;
    #[test]
    #[ignore]
    fn test_server() {
        server::start();
    }
}