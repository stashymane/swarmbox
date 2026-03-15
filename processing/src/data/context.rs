use crate::data::cache::Cache;
use crate::data::stacks::StackDocument;
use crate::processors::configs::ConfigProcessor;
use crate::processors::includes::IncludeProcessor;
use crate::processors::processor::Processor;
use crate::processors::secrets::SecretProcessor;
use crate::yaml::write_yml;
use log::debug;
use saphyr::YamlOwned;
use shared::data::{Config, RelativePath};
use tokio::io::AsyncWriteExt;

pub struct ProcessingContext {
    pub config: Config,
    pub cache: Cache,
    pub processors: Vec<Box<dyn Processor>>,
}

impl ProcessingContext {
    pub async fn load(config: Config) -> Result<ProcessingContext, String> {
        let cache = Cache::load(&config.paths.root, |_path| false).await?;

        let mut processors: Vec<Box<dyn Processor>> = vec![
            Box::new(IncludeProcessor::new()),
            Box::new(ConfigProcessor::new()),
            Box::new(SecretProcessor::new()),
        ];

        for processor in processors.iter_mut() {
            processor.setup(&config).await?;
        }

        Ok(ProcessingContext {
            config,
            cache,
            processors,
        })
    }

    pub async fn process(&self, source_path: &RelativePath) -> Result<(), String> {
        debug!("Processing \"{:?}\"", source_path);
        let mut doc = StackDocument::load(source_path, &self.config)
            .await
            .or_else(|e| {
                Err(format!(
                    "Failed to load stack \"{:?}\": {:?}",
                    source_path, e
                ))
            })?;

        for processor in self.processors.iter() {
            processor.process(&mut doc, &self.config).await?;
        }

        doc.write().await;

        Ok(())
    }
}
