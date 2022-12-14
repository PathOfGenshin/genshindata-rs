// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type NpcFirstMetExcelConfigData = Vec<NpcFirstMetExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcFirstMetExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "subQuestIdList")]
    pub sub_quest_id_list: Vec<i64>,

    #[serde(rename = "avatarID")]
    pub avatar_id: i64,

    #[serde(rename = "avatarDescriptionTextMapHash")]
    pub avatar_description_text_map_hash: i64,
}

pub fn load() -> Result<NpcFirstMetExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "NpcFirstMetExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
