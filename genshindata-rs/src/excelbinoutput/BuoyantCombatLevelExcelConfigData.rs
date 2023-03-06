// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BuoyantCombatLevelExcelConfigData = Vec<BuoyantCombatLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuoyantCombatLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "rewardPreview")]
    pub reward_preview: i64,

    #[serde(rename = "IAIAOJMOAFG")]
    pub iaiaojmoafg: i64,

    #[serde(rename = "BKNHIBMKOHK")]
    pub bknhibmkohk: i64,

    #[serde(rename = "MGAJLAEHLIL")]
    pub mgajlaehlil: Vec<i64>,

    #[serde(rename = "CJDNFCMEDID")]
    pub cjdnfcmedid: Vec<i64>,

    #[serde(rename = "recommendLevel")]
    pub recommend_level: Vec<i64>,

    #[serde(rename = "CBDDJJPEBMP")]
    pub cbddjjpebmp: Vec<f64>,
}

pub fn load() -> Result<BuoyantCombatLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BuoyantCombatLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
