#[async_std::main]
async fn main() {
    if let Err(err) = nucle::run().await {
        panic!("{}", err);
    }
}
