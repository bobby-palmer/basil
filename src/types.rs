use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize)]
#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct BuildTarget {
    id: BuildTargetIdentifier,
    display_name: Option<String>,
    base_directory: Option<URI>,
    tags: Vec<BuildTargetTag>,
    language_ids: Vec<LanguageId>,
    dependencies: Vec<BuildTargetIdentifier>,
    capabilities: BuildTargetCapabilities,
    data_kind: Option<BuildTargetDataKind>,
    data: Option<BuildTargetData>,
}

#[derive(Serialize, Deserialize)]
pub struct BuildTargetIdentifier {
    uri: URI,
}

pub type URI = String;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildTargetTag {
    Application,
    Benchmark,
    IntegrationTest,
    Library,
    Manual,
    NoIde,
    Test,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LanguageId {
    C,
    Cpp,
    Java,
    Python,
    Rust,
    Scala,
    // TODO
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct BuildTargetCapabilities {
    can_compile: Option<bool>,
    can_test: Option<bool>,
    can_run: Option<bool>,
    can_debug: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildTargetDataKind {
    Cargo,
    Cpp,
    Jvm,
    Python,
    Sbt,
    Scala,
}

pub type BuildTargetData = serde_json::Value;

pub type OriginId = String;

#[derive(Serialize, Deserialize)]
#[skip_serializing_none]
pub struct TaskId {
    id: Identifier,
    parents: Option<Vec<Identifier>>,
}

pub type Identifier = String;

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StatusCode {
    Ok,
    Error,
    Cancelled,
}

pub type EnvironmentVariables = serde_json::Map<String, String>;
