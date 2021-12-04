// Copyright (c) 2021 TVWS-Project

use crate::types::*;
use mongodb::{options::ClientOptions, Client};
use redis;
use std::error::Error;

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

// Mongodb docker container must be running
async fn mongodb_connect() -> mongodb::error::Result<()> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}

// TODO
// If the location is outside all regulatory domain supported by the
// Database, the Database MUST respond with an OUTSIDE_COVERAGE error
// Use reserve geocoding: convert coordinates to country
// Reference: https://www.geeksforgeeks.org/how-to-check-if-a-given-point-lies-inside-a-polygon

pub fn reverse_geocode(_latitude: Float, _longitude: Float) -> Result<String, Box<dyn Error>> {
    Ok("ng".to_string())
}
