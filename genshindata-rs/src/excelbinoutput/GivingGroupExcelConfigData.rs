/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GivingGroupExcelConfigData = Vec<GivingGroupExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GivingGroupExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "ItemIds")]
    pub item_ids: Vec<i64>,
    pub finish_talk_id: Option<i64>,
    pub mistake_talk_id: Option<i64>,
    pub finish_dialog_id: Option<i64>,
}
