use crate::data::stacks::StackDocument;
use async_trait::async_trait;
use shared::data::Config;

#[async_trait]
pub trait Processor {
    async fn setup(&mut self, config: &Config) -> anyhow::Result<()>;
    async fn process(&self, doc: &mut StackDocument, config: &Config) -> anyhow::Result<()>;
}
