use redis::Commands;

pub fn get_rule_set(rule_set: &str) -> String {
    let mut conn = redis_connect();
    let result: String = redis::cmd("GET")
        .arg(rule_set)
        .query(&mut conn)
        .expect("Failed to GET ruleset");
    result
}

// Redis docker container must be running
fn redis_connect() -> redis::Connection {
    redis::Client::open("redis://localhost:6379")
        .expect("Redis DB connection error")
        .get_connection()
        .expect("Failed to connect to redis")
}
