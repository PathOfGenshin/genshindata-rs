/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DisplayItemExcelConfigData = Vec<DisplayItemExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayItemExcelConfigDatum {
    pub type_desc_text_map_hash: i64,
    pub rank_level: i64,
    pub display_type: Option<DisplayType>,
    pub id: i64,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    pub icon: String,
    pub item_type: ItemType,
    pub param: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisplayType {
    #[serde(rename = "BARTENDER_ITEM")]
    BartenderItem,
    #[serde(rename = "DEFAULT_ITEM")]
    DefaultItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemType {
    #[serde(rename = "ITEM_DISPLAY")]
    ItemDisplay,
}
