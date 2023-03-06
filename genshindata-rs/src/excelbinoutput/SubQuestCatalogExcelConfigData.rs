// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SubQuestCatalogExcelConfigData = Vec<SubQuestCatalogExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubQuestCatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "OBEMIEIODDM")]
    pub obemieioddm: Obemieioddm,

    #[serde(rename = "FMLHNMLKEGI")]
    pub fmlhnmlkegi: Vec<Fmlhnmlkegi>,

    #[serde(rename = "JDDOGCEHGHF")]
    pub jddogcehghf: Vec<Fmlhnmlkegi>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "CEFDKFMCMMP")]
    pub cefdkfmcmmp: Option<Obemieioddm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fmlhnmlkegi {
    #[serde(rename = "type")]
    pub fmlhnmlkegi_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Obemieioddm {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,

    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_CATALOG_COND_TYPE_QUEST")]
    QuestCatalogCondTypeQuest,
}

pub fn load() -> Result<SubQuestCatalogExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "SubQuestCatalogExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
