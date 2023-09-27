/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FurnitureMakeExcelConfigData = Vec<FurnitureMakeExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FurnitureMakeExcelConfigDatum {
    #[serde(rename = "configID")]
    pub config_id: i64,
    #[serde(rename = "furnitureItemID")]
    pub furniture_item_id: i64,
    pub count: i64,
    pub exp: i64,
    pub material_items: Vec<MaterialItem>,
    pub make_time: i64,
    pub max_accelerate_time: i64,
    pub quick_fetch_material_num: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialItem {
    pub id: Option<i64>,
    pub count: Option<i64>,
}
