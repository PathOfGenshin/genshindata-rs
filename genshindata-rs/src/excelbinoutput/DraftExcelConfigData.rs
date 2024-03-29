/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DraftExcelConfigData = Vec<DraftExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftExcelConfigDatum {
    pub id: i64,
    pub transfer_config: Vec<TransferConfig>,
    pub scene_id: Option<i64>,
    pub exec: String,
    pub param: Option<i64>,
    pub enable_mp: bool,
    pub is_need_all_agree: bool,
    pub confirm_count_down: i64,
    pub min_player_count: i64,
    pub is_need_twice_confirm: Option<bool>,
    pub twice_confirm_count_down: i64,
    pub exec_sub_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferConfig {
    pub group_id: Option<i64>,
    pub config_id: Option<i64>,
}
