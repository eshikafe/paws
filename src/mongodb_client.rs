// Copyright (c) 2021 TVWS-Project
// Main database for the SpectrumDB

use crate::types::*;
use mongodb::{options::ClientOptions, Client};

// Redis docker container must be running
fn mongodb_connect() {
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
}

// TODO
// If the location is outside all regulatory domain supported by the
// Database, the Database MUST respond with an OUTSIDE_COVERAGE error
// Use reserve geocoding: convert coordinates to country
// Reference: https://www.geeksforgeeks.org/how-to-check-if-a-given-point-lies-inside-a-polygon

pub fn reverse_geocode(latitude: Float, longitude: Float) -> Result<String, Error> {
    retrun "ng".to_string();
}

