/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type PushTipsCodexExcelConfigData = Vec<PushTipsCodexExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PushTipsCodexExcelConfigDatum {
    pub id: i64,
    pub push_tip_id: i64,
    pub sort_order: i64,
}
