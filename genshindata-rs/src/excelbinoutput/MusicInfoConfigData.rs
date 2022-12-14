// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MusicInfoConfigData = Vec<MusicInfoConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicInfoConfigDatum {
    #[serde(rename = "musicID")]
    pub music_id: i64,

    #[serde(rename = "musicTime")]
    pub music_time: i64,

    #[serde(rename = "levelId")]
    pub level_id: Vec<i64>,

    #[serde(rename = "musicNameTextMapHash")]
    pub music_name_text_map_hash: i64,

    #[serde(rename = "musicDescTextMapHash")]
    pub music_desc_text_map_hash: i64,

    #[serde(rename = "BGBDMJCFBJD")]
    pub bgbdmjcfbjd: i64,

    #[serde(rename = "MHJEJCFFFGO")]
    pub mhjejcfffgo: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "pointID")]
    pub point_id: i64,

    #[serde(rename = "IBPGODMHPIG")]
    pub ibpgodmhpig: i64,
}

pub fn load() -> Result<MusicInfoConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MusicInfoConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
