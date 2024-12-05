use api_rust::init::initialize;

#[tokio::main]
async fn main() {
    initialize().await;
}
