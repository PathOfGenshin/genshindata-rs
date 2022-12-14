// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type SubQuestCatalogExcelConfigData = Vec<SubQuestCatalogExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubQuestCatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "KFKAEMIEMLM")]
    pub kfkaemiemlm: Kfkaemiemlm,

    #[serde(rename = "CKPIACLNHBP")]
    pub ckpiaclnhbp: Vec<Ckpiaclnhbp>,

    #[serde(rename = "PDGKEMPCDIN")]
    pub pdgkempcdin: Vec<Ckpiaclnhbp>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "PJADJDNALON")]
    pub pjadjdnalon: Option<Kfkaemiemlm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ckpiaclnhbp {
    #[serde(rename = "type")]
    pub ckpiaclnhbp_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_CATALOG_COND_TYPE_QUEST")]
    QuestCatalogCondTypeQuest,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Kfkaemiemlm {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,

    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}

pub fn load() -> Result<SubQuestCatalogExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "SubQuestCatalogExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
