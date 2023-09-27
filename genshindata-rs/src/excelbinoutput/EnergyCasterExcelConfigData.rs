/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EnergyCasterExcelConfigData = Vec<EnergyCasterExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyCasterExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "AFEEIDDFHEO")]
    pub afeeiddfheo: String,
    pub trigger_type: String,
    #[serde(rename = "PGJLAHNJMAH")]
    pub pgjlahnjmah: Option<i64>,
    pub action_type: String,
    #[serde(rename = "DLIEJBOIOOA")]
    pub dliejboiooa: Vec<Option<serde_json::Value>>,
    pub ability_name: String,
}
