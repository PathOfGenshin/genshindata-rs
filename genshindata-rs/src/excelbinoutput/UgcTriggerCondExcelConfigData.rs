/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcTriggerCondExcelConfigData = Vec<UgcTriggerCondExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UgcTriggerCondExcelConfigDatum {
    pub id: i64,
    pub cond_type: String,
    #[serde(rename = "AINCBDPMAPJ")]
    pub aincbdpmapj: i64,
}
