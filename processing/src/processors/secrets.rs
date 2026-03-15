use crate::data::context::Context;
use crate::data::stacks::StackDocument;
use crate::yaml::MappingExt;
use saphyr::YamlOwned;

pub async fn process_secrets(doc: &mut StackDocument, _context: &Context) -> Result<(), String> {
    //TODO
    let yaml = &doc.root;

    let Some(_services) = yaml
        .get_value("services")
        .map(YamlOwned::as_mapping)
        .flatten()
    else {
        return Ok(());
    };

    Ok(())
}
