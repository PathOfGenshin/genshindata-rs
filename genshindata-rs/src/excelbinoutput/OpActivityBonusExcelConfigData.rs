// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type OpActivityBonusExcelConfigData = Vec<OpActivityBonusExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpActivityBonusExcelConfigDatum {
    #[serde(rename = "bonusId")]
    pub bonus_id: i64,

    #[serde(rename = "sourceType")]
    pub source_type: SourceType,

    #[serde(rename = "sourceParam")]
    pub source_param: String,

    #[serde(rename = "openLevel")]
    pub open_level: i64,

    #[serde(rename = "bonusRatio")]
    pub bonus_ratio: i64,

    #[serde(rename = "textMapIdList")]
    pub text_map_id_list: Vec<String>,

    #[serde(rename = "trackPara")]
    pub track_para: Vec<Option<serde_json::Value>>,

    #[serde(rename = "MEAMBLBIKMM")]
    pub meamblbikmm: i64,

    #[serde(rename = "LBHKPPEIKDP")]
    pub lbhkppeikdp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SourceType {
    #[serde(rename = "SOURCE_TYPE_BLOSSOM")]
    SourceTypeBlossom,

    #[serde(rename = "SOURCE_TYPE_DUNGEON")]
    SourceTypeDungeon,
}

pub fn load() -> Result<OpActivityBonusExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "OpActivityBonusExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
