use futures_util::StreamExt;
use kube::api::Api;
use kube::runtime::controller::{self, Action};
use kube::runtime::reflector::ObjectRef;
use kube::runtime::watcher;
use kube::runtime::Controller;
use kube::{Client, ResourceExt};
use sample_custom_controller::Sample;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let client = Client::try_default().await?;
    let samples: Api<Sample> = Api::all(client.clone());
    let controller = Controller::new(samples, watcher::Config::default());

    let context = Arc::new(());
    controller
        .shutdown_on_signal()
        .run(reconcile, on_error, context)
        .for_each(on_completed)
        .await;

    Ok(())
}

async fn reconcile(resource: Arc<Sample>, _: Arc<()>) -> kube::Result<Action> {
    log::info!("Start {}", resource.name_any());
    Ok(Action::await_change())
}

fn on_error(resource: Arc<Sample>, error: &kube::Error, _: Arc<()>) -> Action {
    log::error!("Err {} {:?}", resource.name_any(), error);
    Action::requeue(Duration::from_secs(3))
}

async fn on_completed(
    result: Result<(ObjectRef<Sample>, Action), controller::Error<kube::Error, watcher::Error>>,
) {
    match result {
        Ok((resource, action)) => {
            log::info!("Ok {} {:?}", resource.name, action);
        }
        Err(e) => {
            log::error!("Err {e:?}");
        }
    }
}
