// PAWS - Protocol to Access White Space Database
// RFC 7545

// pub mod parameters;
// pub mod methods;
// pub mod errors;
// pub mod types;
// pub mod version;
pub mod redis_client;

#[cfg(test)]
mod paws_tests {
    use super::redis_client as rc;
    #[test]
    fn test_redis_db() {
        let r1 = rc::get_ruleset("nccId");
        let r2 = rc::get_ruleset("fccId");
        assert_eq!("123", r1);
        assert_eq!("12345", r2);
        println!("value for nccId = {}", r1);
        println!("value for fccId = {}", r2);
        
    }
}