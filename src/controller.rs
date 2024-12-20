use crate::config::{Label, TaintConfig};
use k8s_openapi::api::core::v1::{Node, NodeSpec, Taint};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::{
    api::{Patch, PatchParams},
    runtime::watcher,
    runtime::WatchStreamExt,
    Api,
};
use log::{debug, error, info};
use tokio_stream::StreamExt;

// This is the main controller function.
pub async fn controller(client: kube::Client, config: TaintConfig) {
    let node_api: Api<Node> = Api::all(client);

    loop {
        info!("starting watching nodes");
        let stream = watcher(node_api.clone(), watcher::Config::default()).applied_objects();
        let mut stream = std::pin::pin!(stream);
        let mut next = stream.try_next().await;

        while let Ok(Some(node)) = next {
            for label_taint in &config.label_taints {
                if !node_matches_labels(&label_taint.selector, &node) {
                    continue;
                }

                let err = process_event_for_taint(&node_api, &node, &label_taint.taint).await;
                if let Err(e) = err {
                    error!("error processing event: {}", e);
                }
            }

            next = stream.try_next().await;
        }

        if let Err(e) = next {
            error!("stream terminated with: {}", e);
        }
        info!("watcher stream terminated, restarting in 10 seconds");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

fn node_matches_labels(label: &Label, node: &Node) -> bool {
    let labels = match &node.metadata.labels {
        Some(labels) => labels,
        None => return false,
    };

    let entry = match labels.get(&label.key) {
        Some(entry) => entry.to_owned(),
        None => return false,
    };

    if label.value.is_none() {
        return true;
    }

    Some(entry) == label.value
}

async fn process_event_for_taint(
    api: &Api<Node>,
    node: &Node,
    taint: &Taint,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = match node.spec.clone() {
        Some(spec) => spec,
        _ => {
            debug!("no spec");
            return Ok(());
        }
    };

    let taints = spec.taints.unwrap_or_default();

    if contains_taint(&taints, taint) {
        debug!("taint already present");
        return Ok(());
    }

    let patch = patch_taints(node.clone(), taint);

    let patch_params = PatchParams::apply("taint-controller");
    api.patch(node.metadata.name.as_ref().unwrap(), &patch_params, &patch)
        .await?;
    info!("taint added");

    Ok(())
}

fn contains_taint(taints: &Vec<Taint>, taint: &Taint) -> bool {
    for present_taint in taints {
        if present_taint.key == taint.key
            && present_taint.value == taint.value
            && present_taint.effect == taint.effect
        {
            return true;
        }
    }

    false
}

fn patch_taints(node: Node, taint: &Taint) -> Patch<Node> {
    let mut taints = node
        .spec
        .and_then(|spec| spec.taints.clone())
        .unwrap_or_default();
    taints.push(taint.clone());
    let patch = Node {
        metadata: ObjectMeta {
            name: node.metadata.name.clone(),
            namespace: node.metadata.namespace.clone(),
            ..ObjectMeta::default()
        },
        spec: Some(NodeSpec {
            taints: Some(taints),
            ..NodeSpec::default()
        }),
        status: None,
    };
    Patch::Apply(patch)
}
