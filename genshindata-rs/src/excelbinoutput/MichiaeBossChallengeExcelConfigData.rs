/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MichiaeBossChallengeExcelConfigData = Vec<MichiaeBossChallengeExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MichiaeBossChallengeExcelConfigDatum {
    #[serde(rename = "levelID")]
    pub level_id: i64,
    pub level_title_text_map_hash: i64,
    pub level_desc_text_map_hash: i64,
    #[serde(rename = "groupID")]
    pub group_id: i64,
}
