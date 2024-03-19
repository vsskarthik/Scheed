use axum::{routing::get, Router};
mod routes;

pub async fn start_server() {
    // build our application with a single route
    let app = Router::new().route("/", get(routes::root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2805").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
