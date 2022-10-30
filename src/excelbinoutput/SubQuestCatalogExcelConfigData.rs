// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type SubQuestCatalogExcelConfigData = Vec<SubQuestCatalogExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubQuestCatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "IKGENMHGIOE")]
    pub ikgenmhgioe: Ikgenmhgioe,

    #[serde(rename = "HCNONMEJIBF")]
    pub hcnonmejibf: Vec<Hcnonmejibf>,

    #[serde(rename = "KKCCBFMMDIP")]
    pub kkccbfmmdip: Vec<Hcnonmejibf>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "LHMNPKCPCDH")]
    pub lhmnpkcpcdh: Option<Ikgenmhgioe>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hcnonmejibf {
    #[serde(rename = "type")]
    pub hcnonmejibf_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_CATALOG_COND_TYPE_QUEST")]
    QuestCatalogCondTypeQuest,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ikgenmhgioe {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,

    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}

pub fn load() -> Result<SubQuestCatalogExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "SubQuestCatalogExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
