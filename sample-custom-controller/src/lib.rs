use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, CustomResource, Debug, Deserialize, JsonSchema, Serialize)]
#[kube(
    kind = "Sample",
    group = "sample.custom-controller",
    version = "v1alpha1",
    namespaced
)]
#[kube(status = "SampleStatus")]
pub struct SampleSpec {
    pub name: String,
}

// -----------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleStatus {
    pub check: bool,
}

// -----------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceStatus<T> {
    pub status: T,
}
