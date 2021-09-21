// Copyright (c) 2021 TVWS-Project

use redis::Commands;

pub fn get_ruleset(rule_set: &str) -> String {
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

// Spectrum database
// Channel number: 2,7,9 
// Frequency range (MHz) 470-476, 482-488
// Allowable antenna height (meters AGL): 30, 29.9
// Noise floor (dBm)