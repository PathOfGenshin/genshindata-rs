/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type TargetShootingTargetExcelConfigData = Vec<TargetShootingTargetExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetShootingTargetExcelConfigDatum {
    pub id: i64,
    pub gadget_id: Option<i64>,
    pub target_score: Option<i64>,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    #[serde(rename = "IFKMAANOOBH")]
    pub ifkmaanoobh: Option<bool>,
    #[serde(rename = "IABJJLNINPN")]
    pub iabjjlninpn: Option<f64>,
    #[serde(rename = "OIFJBDGOJHB")]
    pub oifjbdgojhb: Option<bool>,
    #[serde(rename = "LHKFJNFGMFN")]
    pub lhkfjnfgmfn: Option<bool>,
}
