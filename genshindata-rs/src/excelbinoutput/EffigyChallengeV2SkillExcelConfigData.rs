/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EffigyChallengeV2SkillExcelConfigData = Vec<EffigyChallengeV2SkillExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffigyChallengeV2SkillExcelConfigDatum {
    pub id: i64,
    pub ability_name: String,
    #[serde(rename = "OCFOKCEKLCH")]
    pub ocfokceklch: f64,
    pub skill_name_text_map_hash: i64,
    pub skill_desc_text_map_hash: i64,
    pub desc_param: Vec<f64>,
}
