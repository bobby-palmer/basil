use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::server::{
    bsp_types::{LanguageId, URI},
    request_handler::WithParamsHandler,
};

pub struct InitializeBuildHandler;

impl WithParamsHandler<InitializeBuildParams, InitializeBuildResult> for InitializeBuildHandler {
    async fn handle(
        ctx: &crate::server::context::Context,
        params: InitializeBuildParams,
    ) -> Result<InitializeBuildResult, crate::server::error::BasilError> {
        todo!()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitializeBuildParams {
    /// Name of the client
    display_name: String,

    /// The version of the client
    version: String,

    /// The BSP version that the client speaks
    bsp_version: String,

    /// The rootUri of the workspace
    root_uri: URI,

    /// The capabilities of the client
    capabilities: BuildClientCapabilities,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildClientCapabilities {
    /// The languages that this client supports.
    /// The ID strings for each language is defined in the LSP.
    /// The server must never respond with build targets for other
    /// languages than those that appear in this list
    language_ids: Vec<LanguageId>,

    /// Mirror capability to BuildServerCapabilities.jvmCompileClasspathProvider
    /// The client will request classpath via `buildTarget/jvmCompileClasspath` so
    /// it's safe to return classpath in ScalacOptionsItem empty
    jvm_compile_classpath_receiver: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InitializeBuildResult {
    /// Name of the server
    display_name: String,

    /// The version of the server
    version: String,

    /// The BSP version that the server speaks
    bsp_version: String,

    /// The capabilities of the build server
    capabilities: BuildServerCapabilities,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
struct BuildServerCapabilities {
    /// The languages the server supports compilation
    /// via method buildTarget/compile
    compile_provider: Option<Provider>,

    /// The languages the server supports test execution
    /// via method buildTarget/test
    test_provider: Option<Provider>,

    /// The languages the server supports run via
    /// method buildTarget/run
    run_provider: Option<Provider>,

    /// The languages the server supports debugging
    /// via method debugSession/start
    debug_provider: Option<Provider>,

    /// The server can provide a list of targets that contain a
    /// single text document via the method buildTarget/inverseSources
    inverse_sources_provider: Option<bool>,

    /// The server provides sources for library dependencies
    /// via method buildTarget/dependencySources
    dependency_sources_provider: Option<bool>,

    /// The server can provide a list of dependency modules
    /// (libraries with meta information) via method buildTarget/dependencyModules
    dependency_modules_provider: Option<bool>,

    /// The server provides all the resource dependencies
    /// via method buildTarget/resources
    resources_provider: Option<bool>,

    /// The server provides all output paths
    /// via method buildTarget/outputPaths
    output_paths_provider: Option<bool>,

    /// The server sends notifications to the client on build
    /// target change events via buildTarget/didChange
    build_target_changed_provider: Option<bool>,

    /// The server can respond to `buildTarget/jvmRunEnvironment` requests
    /// with the necessary information required to launch a Java process to
    /// run a main class
    jvm_run_environment_provider: Option<bool>,

    /// The server can respond to `buildTarget/jvmTestEnvironment`
    /// requests with the necessary information required to launch a
    /// Java process for testing or debugging
    jvm_test_environment_provider: Option<bool>,

    /// The server can respond to `workspace/cargoFeaturesState` and
    /// `setCargoFeatures` requests. In other words, supports Cargo
    /// Features extension
    cargo_features_provider: Option<bool>,

    /// Reloading the build state through workspace/reload is supported
    can_reload: Option<bool>,

    /// The server can respond to `buildTarget/jvmCompileClasspath` requests
    /// with the necessary information about the target's classpath
    jvm_compile_classpath_provider: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Provider {
    language_ids: Vec<LanguageId>,
}
