pub struct BuildExitHandler;

impl super::NoParamsHandler for BuildExitHandler {
    async fn handle(ctx: &crate::server::context::Context) {
        // std::process::exit(0);
        todo!()
    }
}
