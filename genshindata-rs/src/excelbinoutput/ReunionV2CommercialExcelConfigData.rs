/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReunionV2CommercialExcelConfigData = Vec<ReunionV2CommercialExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReunionV2CommercialExcelConfigDatum {
    pub id: i64,
    pub domestic_link: String,
    pub oversea_link: String,
}
