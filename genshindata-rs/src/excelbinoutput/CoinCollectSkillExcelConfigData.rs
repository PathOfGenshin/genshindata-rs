/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CoinCollectSkillExcelConfigData = Vec<CoinCollectSkillExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinCollectSkillExcelConfigDatum {
    pub id: i64,
    pub ability_name: Vec<String>,
    #[serde(rename = "FHADOCOEJLK")]
    pub fhadocoejlk: Option<bool>,
    #[serde(rename = "AMMEPONAPBA")]
    pub ammeponapba: String,
    pub skill_name_text_map_hash: i64,
    pub skill_desc_text_map_hash: i64,
    #[serde(rename = "JBMLINKIHKB")]
    pub jbmlinkihkb: Option<bool>,
}
