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

pub struct BuildTargetIdentifier {
    uri: URI,
}

pub type URI = String;

pub enum BuildTargetTag {
    Application,
    Benchmark,
    IntegrationTest,
    Library,
    Manual,
    NoIde,
    Test,
}

pub enum LanguageId {
    // TODO
}

pub struct BuildTargetCapabilities {
    can_compile: Option<bool>,
    can_test: Option<bool>,
    can_run: Option<bool>,
    can_debug: Option<bool>,
}

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

pub struct TaskId {
    id: Identifier,
    parents: Option<Vec<Identifier>>,
}

pub type Identifier = String;

pub enum StatusCode {
    Ok,
    Error,
    Cancelled,
}

pub type EnvironmentVariables = serde_json::Map<String, String>;
