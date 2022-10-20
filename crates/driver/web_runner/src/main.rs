use driver_web_common::di::build_di;
use driver_web_rest::app::make_app;

use axum::Extension;

#[tokio::main]
async fn main() {
    let di = build_di().unwrap();

    let rest_app = make_app().layer(Extension(di));

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(rest_app.into_make_service())
        .await
        .unwrap();
}
