/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TreasureMapRegionExcelConfigData = Vec<TreasureMapRegionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreasureMapRegionExcelConfigDatum {
    pub id: i64,
    pub unlock_day: i64,
    pub token_num: i64,
    pub region_center: Vec<f64>,
    pub region_radius: i64,
    pub region_entry_id: i64,
    pub group_list: Vec<i64>,
    pub revise_level: i64,
    pub spot_num_list: Vec<i64>,
    pub token_num_list: Vec<i64>,
    pub misc_drop_prob_list: Vec<i64>,
    pub misc_drop_id: i64,
    pub backup_group_list: Vec<i64>,
    pub mp_token_threshold: i64,
    #[serde(rename = "CPIOKCOHHJG")]
    pub cpiokcohhjg: Vec<f64>,
    #[serde(rename = "JIIOAONMFKJ")]
    pub jiioaonmfkj: i64,
    #[serde(rename = "CJGNEDJGENN")]
    pub cjgnedjgenn: Option<i64>,
    pub mp_group_id: Option<i64>,
    pub mp_type_id: Option<i64>,
    #[serde(rename = "PBCNLEDCJAG")]
    pub pbcnledcjag: Option<i64>,
}
