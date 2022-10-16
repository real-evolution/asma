#[tokio::main]
async fn main() {
    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(host_rest_axum::router::make_router().into_make_service())
        .await
        .unwrap();
}
