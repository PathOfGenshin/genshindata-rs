/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcMaterialEditLimitExcelConfigData = Vec<UgcMaterialEditLimitExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UgcMaterialEditLimitExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub mccmgpjaaok: Vec<Mccmgpjaaok>,
    pub kinfjijanja: Option<i64>,
    pub glkllfampdo: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Mccmgpjaaok {
    #[serde(rename = "materialId")]
    pub material_id: Option<i64>,
    pub edmiaemidfj: Option<i64>,
}
