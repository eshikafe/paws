use warp::Filter;

#[tokio::main]
pub async fn start() {
    // GET /spectrumdb/version => 200 OK with body "TVWS project, warp!"
    let hello = warp::path!("spectrumdb" / String)
        .map(|name| format!("TVWS project, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
