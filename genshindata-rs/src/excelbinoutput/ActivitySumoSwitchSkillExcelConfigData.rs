/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySumoSwitchSkillExcelConfigData = Vec<ActivitySumoSwitchSkillExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySumoSwitchSkillExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub ability_group_name: String,
    pub title_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub desc_param: Vec<String>,
    pub icon_name_hash: f64,
}
