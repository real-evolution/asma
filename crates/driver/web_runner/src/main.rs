use axum::Extension;

mod di;

#[tokio::main]
async fn main() {
    let module = di::build_di_module();

    let rest_app =
        driver_web_rest::router::make_router().layer(Extension(module));

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(rest_app.into_make_service())
        .await
        .unwrap();
}
