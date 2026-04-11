fn main() {
    tracing_subscriber::fmt::init();
    println!("Hello, world!");
    tracing::info!("info");
    tracing::warn!("warn");
    tracing::error!("error");
}
