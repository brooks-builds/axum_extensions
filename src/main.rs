use axum_extensions::App;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    let port = dotenv!("PORT")
        .parse()
        .expect("PORT must be a valid u16 between 1024 and 65535");

    let app = App::new(port);

    app.run().await.expect("Server shut down with error");
}
