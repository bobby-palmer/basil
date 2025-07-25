use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_with::skip_serializing_none;

/// a bazel target corresponds one-to-one with a BuildTarget
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct BuildTarget {
    /// The target's unique identifier
    id: BuildTargetIdentifier,

    /// A human readable name for this target.
    /// May be presented in the user interface.
    /// Should be unique if possible.
    /// The id.uri is used if None.
    display_name: Option<String>,

    /// The directory where this target belongs to. Multiple build targets are
    /// allowed to map to the same base directory, and a build target is not
    /// required to have a base directory. A base directory does not determine
    /// the sources of a target, see buildTarget/sources
    base_directory: Option<URI>,

    /// Free-form string tags to categorize or label this build target.
    /// For example, can be used by the client to:
    /// - customize how the target should be translated into the client's
    ///   project model.
    /// - group together different but related targets in the user interface.
    /// - display icons or colors in the user interface.
    /// Pre-defined tags are listed in `BuildTargetTag` but clients and servers
    /// are free to define new tags for custom purposes
    tags: Vec<BuildTargetTag>,

    /// The set of languages that this target contains.
    /// The ID string for each language is defined in the LSP.
    language_ids: Vec<LanguageId>,

    /// The direct upstream build target dependencies of this build target
    dependencies: Vec<BuildTargetIdentifier>,

    /// The capabilities of this build target
    capabilities: BuildTargetCapabilities,

    /// Kind of data to expect in the `data` field.
    /// If this field is not set, the kind of data is not specified
    data_kind: Option<BuildTargetDataKind>,

    /// Language-specific metadata about this target.
    /// See ScalaBuildTarget as an example
    data: Option<BuildTargetData>,
}

#[derive(Serialize, Deserialize)]
pub struct BuildTargetIdentifier {
    /// The target's Uri
    uri: URI,
}

pub type URI = String;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildTargetTag {
    Preset(PresetBuildTargetTag),
    Custom(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PresetBuildTargetTag {
    /// Target contains source code for producing any kind of application, may
    /// have but does not require the `canRun` capability
    Application,

    /// Target contains source code to measure performance of a program, may
    /// have but does not require the `canRun` build target capability
    Benchmark,

    /// Target contains source code for integration testing purposes, may have
    /// but does not require the `canTest` capability.
    /// The difference between "test" and "integration-test" is that
    /// integration tests traditionally run slower compared to normal tests
    /// and require more computing resources to execute
    IntegrationTest,

    /// Target contains re-usable functionality for downstream targets. May
    /// have any combination of capabilities
    Library,

    /// Actions on the target such as build and test should only be invoked
    /// manually and explicitly. For example, triggering a build on all
    /// targets in the workspace should by default not include this target.
    /// The original motivation to add the "manual" tag comes from a similar
    /// functionality that exists in Bazel, where targets with this tag have
    /// to be specified explicitly on the command line
    Manual,

    /// Target should be ignored by IDEs
    NoIde,

    /// Target contains source code for testing purposes, may have but does not
    /// require the `canTest` capability
    Test,
}

/// Language IDs defined by LSP spec
#[derive(Serialize, Deserialize)]
pub enum LanguageId {
    // TODO add languages
}

/// Clients can use these capabilities to notify users what BSP
/// endpoints can and cannot be used and why
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct BuildTargetCapabilities {
    /// this target can be compiled by the BSP server
    can_compile: Option<bool>,

    /// This target can be tested by the BSP server
    can_test: Option<bool>,

    /// This target can be run by the BSP server
    can_run: Option<bool>,

    /// This target can be debugged by the BSP server
    can_debug: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildTargetDataKind {
    /// `data` field must contain a CargoBuildTarget object
    Cargo,

    /// `data` field must contain a CppBuildTarget object
    Cpp,

    /// `data` field must contain a JvmBuildTarget object
    Jvm,

    /// `data` field must contain a PythonBuildTarget object
    Python,

    /// `data` field must contain a SbtBuildTarget object
    Sbt,

    /// `data` field must contain a ScalaBuildTarget object
    Scala,
}

pub type BuildTargetData = Value;

/// Represents the identifier of a BSP request.
pub type OriginId = String;

/// The Task Id allows clients to uniquely identify a BSP task and establish
/// a client-parent relationship with another task id.
#[derive(Serialize, Deserialize)]
#[skip_serializing_none]
pub struct TaskId {
    /// A unique identifier
    id: Identifier,

    /// The parent task ids, if any. A non-empty parents field means
    /// this task is a sub-task of every parent task id. The child-parent
    /// relationship of tasks makes it possible to render tasks in
    /// a tree-like user interface or inspect what caused a certain task
    /// execution. OriginId should not be included in the parents field,
    /// there is a separate field for that
    parents: Option<Vec<Identifier>>,
}

pub type Identifier = String;

/// Included in notifications of tasks or
/// requests to signal the completion state
pub enum StatusCode {
    Ok,
    Error,
    Cancelled,
}

pub type EnvironmentVariables = Map<String, String>;
