/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type OfferingVersionExcelConfigData = Vec<OfferingVersionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferingVersionExcelConfigDatum {
    pub id: i64,
    pub offering_id: i64,
    #[serde(rename = "IMFLHGMKGDE")]
    pub imflhgmkgde: i64,
}
