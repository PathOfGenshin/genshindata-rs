// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriExcelConfigData = Vec<IrodoriExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriExcelConfigDatum {
    #[serde(rename = "PJGEHBLALOO")]
    pub pjgehblaloo: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "HFMPMIGMHFF")]
    pub hfmpmigmhff: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: i64,

    #[serde(rename = "guideQuestId")]
    pub guide_quest_id: Option<i64>,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: Option<i64>,

    #[serde(rename = "IFKKGKLCMFO")]
    pub ifkkgklcmfo: i64,

    #[serde(rename = "CFKMNGOENAP")]
    pub cfkmngoenap: i64,
}

pub fn load() -> Result<IrodoriExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "IrodoriExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
