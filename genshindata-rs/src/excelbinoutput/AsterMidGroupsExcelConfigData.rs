/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AsterMidGroupsExcelConfigData = Vec<AsterMidGroupsExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsterMidGroupsExcelConfigDatum {
    pub id: i64,
    pub group_id: i64,
    pub battle_group_vec: Vec<i64>,
}
