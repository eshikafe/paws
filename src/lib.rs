// (c) 2021 TVWS-Project
// PAWS - Protocol to Access White Space Database
// Compliant with RFC 7545

// pub mod parameters;
// pub mod methods;
// pub mod errors;
// pub mod types;
// pub mod version;
pub mod redis_client;
pub mod server;


#[cfg(test)]
mod paws_tests {
    use super::redis_client as rc;
    use mac_address::get_mac_address;
    
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
    fn get_mac_addr() {
        match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
        }
    }

    use super::server;
    #[test]
    fn test_server() {
        server::start();
    }
}