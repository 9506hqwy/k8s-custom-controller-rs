use futures_util::StreamExt;
use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::{self, Action};
use kube::runtime::finalizer;
use kube::runtime::finalizer::Event;
use kube::runtime::reflector::ObjectRef;
use kube::runtime::watcher;
use kube::runtime::Controller;
use kube::{Client, ResourceExt};
use sample_custom_controller::{ResourceStatus, Sample, SampleStatus};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let client = Client::try_default().await?;
    let samples: Api<Sample> = Api::all(client.clone());
    let controller = Controller::new(samples, watcher::Config::default());

    let context = Arc::new(Context { client });
    controller
        .shutdown_on_signal()
        .run(reconcile, on_error, context)
        .for_each(on_completed)
        .await;

    Ok(())
}

async fn reconcile(resource: Arc<Sample>, context: Arc<Context>) -> kube::Result<Action> {
    log::info!("Start {}", resource.name_any());
    let namespace = resource.namespace().unwrap();
    let samples: Api<Sample> = Api::namespaced(context.client.clone(), &namespace);
    finalizer(
        &samples,
        "sample-custom-controller/v1alpha1",
        resource,
        |event| reconcile_resource(event, context),
    )
    .await
    .map_err(on_error_finalizer)
}

async fn reconcile_resource(event: Event<Sample>, context: Arc<Context>) -> kube::Result<Action> {
    match event {
        Event::Apply(resource) => {
            log::info!("Apply {}", resource.name_any());
            let namespace = resource.namespace().unwrap();
            let samples: Api<Sample> = Api::namespaced(context.client.clone(), &namespace);
            let patch = Patch::Merge(ResourceStatus {
                status: SampleStatus { check: true },
            });
            samples
                .patch_status(
                    resource.name_any().as_str(),
                    &PatchParams::default(),
                    &patch,
                )
                .await?;
        }
        Event::Cleanup(resource) => {
            log::info!("Cleanup {}", resource.name_any());
        }
    }

    Ok(Action::await_change())
}

fn on_error(resource: Arc<Sample>, error: &kube::Error, _: Arc<Context>) -> Action {
    log::error!("Err {} {:?}", resource.name_any(), error);
    Action::requeue(Duration::from_secs(3))
}

fn on_error_finalizer(e: finalizer::Error<kube::Error>) -> kube::Error {
    log::error!("Err {e:?}");
    match e {
        finalizer::Error::AddFinalizer(e) => e,
        finalizer::Error::ApplyFailed(e) => e,
        finalizer::Error::CleanupFailed(e) => e,
        finalizer::Error::InvalidFinalizer => unreachable!(),
        finalizer::Error::RemoveFinalizer(e) => e,
        finalizer::Error::UnnamedObject => unreachable!(),
    }
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

// -----------------------------------------------------------------------------------------------

struct Context {
    client: Client,
}
