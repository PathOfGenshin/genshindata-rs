/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SalvageOverAllExcelConfigData = Vec<SalvageOverAllExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalvageOverAllExcelConfigDatum {
    pub id: i64,
    pub activity_id: i64,
    #[serde(rename = "HBEOJCKEEHH")]
    pub hbeojckeehh: i64,
    pub pre_quest_id: i64,
    pub guide_quest_id: i64,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub region_center: Vec<i64>,
    pub region_radius: i64,
    #[serde(rename = "FPEGHOEJADF")]
    pub fpeghoejadf: i64,
    pub reminder_id: i64,
    #[serde(rename = "BMHGJMAEFJG")]
    pub bmhgjmaefjg: i64,
    pub reward_preview_id: i64,
    #[serde(rename = "FIKEOBMNFEE")]
    pub fikeobmnfee: i64,
    #[serde(rename = "KILLIHIAFDL")]
    pub killihiafdl: i64,
}
