use std::collections::HashMap;

use genshindata_rs::{
    excelbinoutput::{
        EquipAffixExcelConfigData::{self, EquipAffixExcelConfigDatum},
        ReliquarySetExcelConfigData::{self, ReliquarySetExcelConfigDatum},
    },
    json::JsonError,
    language::Language,
    textmap::AllTextMaps,
};
use strum::IntoEnumIterator;

use crate::models::{
    artifact::{ArtifactSet, ArtifactSetTkeys},
    translatable::{TranslatableData, Translations}, traits::{Processable, Translatable},
};

#[derive(Debug)]
pub struct ArtifactSetProcessor {
    reliquary_set_data: ReliquarySetExcelConfigData::ReliquarySetExcelConfigData,
    equip_affix_data: EquipAffixExcelConfigData::EquipAffixExcelConfigData,
}

impl ArtifactSetProcessor {
    pub fn new() -> Result<Self, JsonError> {
        let reliquary_set_data = ReliquarySetExcelConfigData::load()?;
        let equip_affix_data = EquipAffixExcelConfigData::load()?;
        Ok(Self {
            reliquary_set_data,
            equip_affix_data,
        })
    }

    fn translation_keys(&self, reliquary_set: &ReliquarySetExcelConfigDatum) -> ArtifactSetTkeys {
        ArtifactSetTkeys {
            set_name: format!("ArtifactSet:{}:setName", reliquary_set.set_id),
        }
    }
}

impl Processable<Vec<ArtifactSet>> for ArtifactSetProcessor {
    fn process(&self, all_textmaps: &AllTextMaps) -> TranslatableData<Vec<ArtifactSet>> {
        let artifact_sets: Vec<ArtifactSet> = self
            .reliquary_set_data
            .iter()
            .filter(|r| r.disable_filter.is_none() || r.disable_filter.unwrap_or(0) == 0)
            .map(|r| ArtifactSet {
                id: r.set_id,
                set_icon: r.set_icon.clone(),
                equip_affix_group_id: r.equip_affix_id,
                set_activation_count: r.set_need_num.iter().map(|i| *i as i8).collect(),
                artifact_ids: r.contains_list.clone(),
                tkeys: self.translation_keys(r),
            })
            .collect();
        TranslatableData {
            translations: self.translations(&artifact_sets, all_textmaps),
            data: artifact_sets,
        }
    }
}

impl Translatable<Vec<ArtifactSet>> for ArtifactSetProcessor {
    fn translations(&self, data: &Vec<ArtifactSet>, all_textmaps: &AllTextMaps) -> Translations {
        let equip_affix_by_id: HashMap<i64, &EquipAffixExcelConfigDatum> =
            HashMap::from_iter(self.equip_affix_data.iter().map(|affix| (affix.id, affix)));
        let mut translations = Translations::default();
        for artifact_set in data.iter() {
            for language in Language::iter() {
                let artifact_equip_affix_name = equip_affix_by_id
                    .get(&artifact_set.equip_affix_group_id.unwrap_or(0))
                    .map(|affix| affix.name_text_map_hash)
                    .and_then(|name_hash| all_textmaps.get(language).get(name_hash));

                // Save artifact set translation keys
                translations.put(
                    language,
                    artifact_set.tkeys.set_name.clone(),
                    artifact_equip_affix_name.unwrap_or_default(),
                );
            }
        }
        translations
    }
}
