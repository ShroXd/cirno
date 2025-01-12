use actix::Addr;
use ambassador::Delegate;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::*;

use crate::{
    application::file_service::FileService,
    domain::{
        pipeline::event::PipelineEvent,
        task::async_task::{
            ambassador_impl_TaskIdentifiable, AsyncTask, TaskId, TaskIdentifiable, TaskIdentifier,
        },
    },
    infrastructure::{
        dispatcher::{domain_event::DomainEvent, event_bus::EventBus},
        file::finder_options::{all_files, FinderOptions},
        hls::hls_state_actor::HlsStateActor,
    },
};

#[derive(Delegate)]
#[delegate(TaskIdentifiable, target = "identifier")]
pub struct PipelinePreparationTask {
    identifier: TaskIdentifier,
    file_service: Arc<FileService>,
    #[allow(dead_code)]
    event_bus: Arc<EventBus>,
    #[allow(dead_code)]
    hls_state_actor_addr: Addr<HlsStateActor>,
}

#[async_trait]
impl AsyncTask for PipelinePreparationTask {
    async fn execute(&self, _identifier: TaskIdentifier, event_bus: Arc<EventBus>) -> Result<()> {
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
}

impl PipelinePreparationTask {
    pub fn new(
        file_service: Arc<FileService>,
        event_bus: Arc<EventBus>,
        hls_state_actor_addr: Addr<HlsStateActor>,
    ) -> Self {
        Self {
            identifier: TaskIdentifier::default(),
            file_service,
            event_bus,
            hls_state_actor_addr,
        }
    }
}
