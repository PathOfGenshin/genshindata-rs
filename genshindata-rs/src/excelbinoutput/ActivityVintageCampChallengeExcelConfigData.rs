/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityVintageCampChallengeExcelConfigData = Vec<ActivityVintageCampChallengeExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityVintageCampChallengeExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "EOJDFACEGPO")]
    pub eojdfacegpo: i64,
    pub open_day: i64,
    #[serde(rename = "LBKAMKPHAJJ")]
    pub lbkamkphajj: i64,
    pub group_bundle_id: i64,
    pub reward_id: i64,
    #[serde(rename = "IPGMLHIOEGI")]
    pub ipgmlhioegi: Vec<String>,
    pub watcher_ids: Vec<i64>,
    pub monster_info: String,
    pub elite_monster_info: Vec<i64>,
    #[serde(rename = "CFINJJOCEFA")]
    pub cfinjjocefa: i64,
    pub buff_desc_text_map_hash: i64,
}
