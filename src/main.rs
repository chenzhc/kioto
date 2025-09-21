use log::info;

#[tokio::main]
async fn main() {
    kioto::init();

    info!("Hello, world!");
}
