#![feature(never_type)]

use automatea::{GatewayAPI, Error, HttpAPI};

#[tokio::main]
async fn main() -> Result<!, Error> {
    automatea::setup_logging()?;

    let http = HttpAPI::new("NjEzMDUzOTEwMjc3NTU0MTg0.XVrU-Q.-Liuq8tU9HQtNN6pWD-Tjxu7IRY");
    GatewayAPI::connect(http.gateway_bot().await?)
}