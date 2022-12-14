// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LuminanceStoneChallengeStageExcelConfigData = Vec<LuminanceStoneChallengeStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LuminanceStoneChallengeStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "PANHLIEPKDG")]
    pub panhliepkdg: i64,

    #[serde(rename = "DIILCFOFHOK")]
    pub diilcfofhok: i64,

    #[serde(rename = "AFJCNMICNEE")]
    pub afjcnmicnee: i64,

    #[serde(rename = "IDHFFELCOCB")]
    pub idhffelcocb: i64,

    #[serde(rename = "CJDALAANEKA")]
    pub cjdalaaneka: Vec<i64>,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "JNFJNEFPIBA")]
    pub jnfjnefpiba: Vec<i64>,

    #[serde(rename = "MNEJODPKJPO")]
    pub mnejodpkjpo: i64,

    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
}

pub fn load() -> Result<LuminanceStoneChallengeStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LuminanceStoneChallengeStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
