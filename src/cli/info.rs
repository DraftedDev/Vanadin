pub fn info() {
    info!("Running Vanadin v{}", env!("CARGO_PKG_VERSION"));
    info!("{}", env!("CARGO_PKG_DESCRIPTION"));
    info!("Made by {}", env!("CARGO_PKG_AUTHORS"));
}
