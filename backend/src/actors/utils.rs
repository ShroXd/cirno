#[macro_export]
macro_rules! process_pipeline_action {
    ($self:ident, $pipeline_action:ident) => {{
        debug!(
            "WebSocket actor received {} action",
            stringify!($pipeline_action)
        );

        if let Some(pipeline_addr) = $self.pipeline_addr.as_ref() {
            debug!(
                "WebSocket actor sending {} action to pipeline",
                stringify!($pipeline_action)
            );

            if let Err(e) = pipeline_addr.try_send(PipelineAction::$pipeline_action) {
                error!("Failed to forward message to pipeline: {:?}", e);
            }
        }
    }};
}
