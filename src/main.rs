use log::{error, info};
use std::{io::stdout};
use structured_logger::{json::new_writer, Builder};

mod config;

mod controller;
use controller::controller;

#[tokio::main]
async fn main() {
    init_logger();
    let taint_config = config::TaintConfig::try_default();

    info!("starting taint controller");

    info!("loading configuration file");
    if let Err(e) = taint_config {
        error!("error loading configuration {}", e);
        return;
    }

    let taint_config = taint_config.unwrap();

    info!("creating kubernetes client");
    let client = match kube::Client::try_default().await {
        Ok(client) => {
            info!("client created");
            client
        }
        Err(e) => {
            info!("couldn't create client");
            error!("{}", e);
            return;
        }
    };

    info!("starting controller");
    controller(client, taint_config).await;

    info!("controller stopped");
    log::logger().flush();
}

fn init_logger() {
    Builder::with_level("info")
        .with_target_writer("*", new_writer(stdout()))
        .init();
}
