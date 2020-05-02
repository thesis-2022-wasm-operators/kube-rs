use anyhow::Result;
use futures::prelude::*;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client, Config};
use kube_rt::Watcher;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::infer().await?;
    let client = Client::new(config);
    let api = Api::<Pod>::all(client);
    let watcher = Watcher::new(api, ListParams::default());
    // Use try_for_each to fail on first error, use for_each to keep retrying
    watcher.try_for_each(|event| async move { Ok(println!("{:?}", event)) }).await?;
    Ok(())
}
