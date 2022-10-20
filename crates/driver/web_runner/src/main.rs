use axum::Extension;
use driver_web_common::di::build_di;

#[tokio::main]
async fn main() {
    let rest_app =
        driver_web_rest::router::make_router().layer(Extension(build_di()));

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(rest_app.into_make_service())
        .await
        .unwrap();
}
