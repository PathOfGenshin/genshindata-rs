/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BladeDanceBuffExcelConfigData = Vec<BladeDanceBuffExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BladeDanceBuffExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "INGLCBHEMHI")]
    pub inglcbhemhi: i64,
    pub desc_param: Vec<String>,
}
