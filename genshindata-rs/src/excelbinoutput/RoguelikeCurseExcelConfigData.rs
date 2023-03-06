// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RoguelikeCurseExcelConfigData = Vec<RoguelikeCurseExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoguelikeCurseExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "HHJNAJBCFML")]
    pub hhjnajbcfml: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "NGLCNDEDFPP")]
    pub nglcndedfpp: Nglcndedfpp,

    #[serde(rename = "KLEGLBEEJHO")]
    pub kleglbeejho: Option<bool>,

    #[serde(rename = "descParamList")]
    pub desc_param_list: Vec<f64>,

    #[serde(rename = "LALPFMKPAGO")]
    pub lalpfmkpago: Vec<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nglcndedfpp {
    #[serde(rename = "effectType")]
    pub effect_type: String,

    #[serde(rename = "OMAMMMMEEGI")]
    pub omammmmeegi: String,

    #[serde(rename = "HDBJHKKLDIE")]
    pub hdbjhkkldie: String,
}

pub fn load() -> Result<RoguelikeCurseExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RoguelikeCurseExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
