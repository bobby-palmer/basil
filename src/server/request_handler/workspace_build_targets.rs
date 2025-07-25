use serde::Serialize;

use crate::server::bsp_types::BuildTarget;

pub struct WorkspaceBuildTargetsHandler;

impl super::NoParamsHandler for WorkspaceBuildTargetsHandler {
    type Output = WorkspaceBuildTargetsResult;

    async fn handle(
        ctx: &crate::server::context::Context,
    ) -> Result<Self::Output, crate::server::error::BasilError> {
        todo!()
    }
}

#[derive(Serialize)]
pub struct WorkspaceBuildTargetsResult {
    /// The build targets in this workspace that
    /// contain sources with the given language ids
    targets: Vec<BuildTarget>,
}
