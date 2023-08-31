use kube::{Api, runtime::watcher, runtime::WatchStreamExt};
use k8s_openapi::api::core::v1::Node;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
  let client = kube::Client::try_default().await.expect("a kubeconfig must be present");
  let node_api: Api<Node> = Api::all(client);

   let stream = watcher(node_api, watcher::Config::default()).applied_objects();
   let mut stream = std::pin::pin!(stream);

   let mut next = stream.try_next().await;

   while let Ok(Some(node)) = next {
       println!("node change {}", node.metadata.name.or(Some("<no name specified>".to_string())).unwrap());
        next = stream.try_next().await;
   }

   if let Err(e) = next {
       println!("{}", e)
   }
}
