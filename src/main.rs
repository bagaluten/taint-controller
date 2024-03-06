use log::{error, info};
use structured_logger::{async_json::new_writer, Builder};

mod config;

mod controller;
use controller::controller;

#[tokio::main]
async fn main() {
    init_logger();
    let taint_config = config::TaintConfig::try_default();

    if let Err(e) = taint_config {
        error!("error loading configuration {}", e);
        return;
    }

    let taint_config = taint_config.unwrap();

    let client = match kube::Client::try_default().await {
        Ok(client) => {
            info!("client created");
            client
        }
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    controller(client, taint_config).await;
}

fn init_logger() {
    Builder::with_level("info")
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();
}
