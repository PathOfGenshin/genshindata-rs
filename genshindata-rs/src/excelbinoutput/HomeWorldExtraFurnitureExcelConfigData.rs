/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HomeWorldExtraFurnitureExcelConfigData = Vec<HomeWorldExtraFurnitureExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct HomeWorldExtraFurnitureExcelConfigDatum {
    #[serde(rename = "furnitureID")]
    pub furniture_id: i64,
    pub icaamnndcnm: Vec<String>,
}
