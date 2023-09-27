use genshindata_rs::{language::Language, textmap::AllTextMaps};

use crate::{artifact::artifact_set::ArtifactSetProcessor, models::traits::Processable};

mod artifact;
mod models;
mod processor;
mod translation;

fn main() {
    let all_textmaps = AllTextMaps::load_all();
    let artifact_sets = ArtifactSetProcessor::new().unwrap();
    let result = artifact_sets.process(&all_textmaps);
    println!("{:?}", result.data);
    let en_translations = result.translations.get_pack(Language::EN);
    println!(
        "{}",
        serde_json::to_string_pretty(&result.translations).unwrap()
    );
}
