use kube::CustomResourceExt;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let crd = sample_custom_controller::Sample::crd();
    print!("{}", serde_yaml::to_string(&crd)?);
    Ok(())
}
