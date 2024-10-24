use actix::{Actor, Context};

pub struct PipelineActor {}

impl Actor for PipelineActor {
    type Context = Context<Self>;
}
