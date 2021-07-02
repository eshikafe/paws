// PAWS - Protocol to Access White Space Database
// RFC 7545

pub mod parameters;
pub mod methods;
pub mod errors;
pub mod types;
pub mod version;
pub mod db;

#[cfg(test)]
mod tests {
    use super::db;
    #[test]
    fn test_redis_db() {
        let r1 = db::get_rule_set("nccId");
        let r2 = db::get_rule_set("fccId");
        assert_eq!("123", r1);
        assert_eq!("12345", r2);
        println!("value for nccId = {}", r1);
        println!("value for fccId = {}", r2);
        
    }
}