use actix::Addr;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::*;

use crate::{
    application::file_service::FileService,
    domain::pipeline::event::PipelineEvent,
    infrastructure::{
        event_bus::{domain_event::DomainEvent, event_bus::EventBus},
        file::finder_options::{all_files, FinderOptions},
        hls::hls_state_actor::HlsStateActor,
        task_pool::model::AsyncTask,
    },
};

pub struct PipelinePreparationTask {
    file_service: Arc<FileService>,
    event_bus: Arc<EventBus>,
    hls_state_actor_addr: Addr<HlsStateActor>,
    task_id: String,
    ws_client_id: String,
}

#[async_trait]
impl AsyncTask for PipelinePreparationTask {
    async fn execute(
        &self,
        _ws_client_id: String,
        task_id: String,
        event_bus: Arc<EventBus>,
    ) -> Result<()> {
        debug!("Preparing pipeline");

        event_bus
            .publish(DomainEvent::Pipeline(PipelineEvent::PreparationStarted))
            .inspect_err(|e| error!("Failed to publish preparation started event: {}", e))?;

        let options = FinderOptions::new()
            .filters(all_files())
            .include_hidden(true);
        self.file_service
            .delete_files_in_folder("./tmp", options)
            .await
            .inspect_err(|e| error!("Failed to delete files in tmp folder: {}", e))?;

        event_bus
            .publish(DomainEvent::Pipeline(PipelineEvent::PreparationFinished))
            .inspect_err(|e| error!("Failed to publish preparation finished event: {}", e))?;

        Ok(())
    }

    fn set_task_id(&mut self, task_id: String) {
        self.task_id = task_id;
    }

    fn get_task_id(&self) -> String {
        self.task_id.clone()
    }

    fn set_ws_client_id(&mut self, ws_client_id: String) {
        self.ws_client_id = ws_client_id;
    }

    fn get_ws_client_id(&self) -> String {
        self.ws_client_id.clone()
    }
}

impl PipelinePreparationTask {
    pub fn new(
        file_service: Arc<FileService>,
        event_bus: Arc<EventBus>,
        hls_state_actor_addr: Addr<HlsStateActor>,
    ) -> Self {
        Self {
            file_service,
            event_bus,
            hls_state_actor_addr,
            task_id: "".to_string(),
            ws_client_id: "".to_string(),
        }
    }
}
