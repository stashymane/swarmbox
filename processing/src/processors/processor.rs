use crate::data::stacks::StackDocument;
use async_trait::async_trait;
use shared::data::Config;

#[async_trait]
pub trait Processor {
    async fn setup(&mut self, config: &Config) -> Result<(), String>;
    async fn process(&self, doc: &mut StackDocument, config: &Config) -> Result<(), String>;
}
