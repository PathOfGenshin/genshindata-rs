// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RogueDiaryDungeonExcelConfigData = Vec<RogueDiaryDungeonExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RogueDiaryDungeonExcelConfigDatum {
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "groupList")]
    pub group_list: Vec<i64>,

    #[serde(rename = "FIMKOKHLLGM")]
    pub fimkokhllgm: Vec<Fimkokhllgm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fimkokhllgm {
    #[serde(rename = "LDOBCIDOGBH")]
    pub ldobcidogbh: Option<i64>,

    #[serde(rename = "DEHNLGGCGFO")]
    pub dehnlggcgfo: Vec<i64>,

    #[serde(rename = "AJDGLMCBMNP")]
    pub ajdglmcbmnp: Option<i64>,
}

pub fn load() -> Result<RogueDiaryDungeonExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RogueDiaryDungeonExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
