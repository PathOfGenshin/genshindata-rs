/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ExploreExcelConfigData = Vec<ExploreExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExploreExcelConfigDatum {
    #[serde(rename = "materialID")]
    pub material_id: i64,
    pub exp: i64,
}
