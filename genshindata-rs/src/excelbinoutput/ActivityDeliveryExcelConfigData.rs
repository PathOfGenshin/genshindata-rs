/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityDeliveryExcelConfigData = Vec<ActivityDeliveryExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDeliveryExcelConfigDatum {
    pub schedule_id: i64,
    pub daily_config_id_list: Vec<i64>,
    pub need_player_level: i64,
}
