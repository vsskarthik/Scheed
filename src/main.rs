mod api;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    api::start_server().await;
}
