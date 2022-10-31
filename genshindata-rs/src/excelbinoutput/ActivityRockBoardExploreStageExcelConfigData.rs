// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityRockBoardExploreStageExcelConfigData = Vec<ActivityRockBoardExploreStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityRockBoardExploreStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "EAJLPCOPPBP")]
    pub eajlpcoppbp: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "LFFAOCPCGIO")]
    pub lffaocpcgio: String,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "JKBJDCOJDLF")]
    pub jkbjdcojdlf: i64,

    #[serde(rename = "watcherID")]
    pub watcher_id: i64,
}

pub fn load() -> Result<ActivityRockBoardExploreStageExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityRockBoardExploreStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}