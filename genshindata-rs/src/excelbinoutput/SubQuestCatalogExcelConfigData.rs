/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SubQuestCatalogExcelConfigData = Vec<SubQuestCatalogExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubQuestCatalogExcelConfigDatum {
    pub id: i64,
    pub show_logic: Logic,
    pub show_cond: Vec<Cond>,
    pub hide_cond: Vec<Cond>,
    pub desc_text_map_hash: i64,
    pub hide_logic: Option<Logic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "type")]
    pub cond_type: Option<Type>,
    pub param: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "QUEST_CATALOG_COND_TYPE_QUEST")]
    QuestCatalogCondTypeQuest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Logic {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}
