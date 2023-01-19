// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type NewActivityExcelConfigData = Vec<NewActivityExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewActivityExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "activityType")]
    pub activity_type: String,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "activitySceneTag")]
    pub activity_scene_tag: ActivitySceneTag,

    #[serde(rename = "condGroupId")]
    pub cond_group_id: Vec<i64>,

    #[serde(rename = "watcherId")]
    pub watcher_id: Vec<i64>,

    #[serde(rename = "PPDLJLHLGAD")]
    pub ppdljlhlgad: Vec<i64>,

    #[serde(rename = "dungeonIdList")]
    pub dungeon_id_list: Vec<i64>,

    #[serde(rename = "isLoadTerrain")]
    pub is_load_terrain: Option<bool>,

    #[serde(rename = "PLAANLBECKI")]
    pub plaanlbecki: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivitySceneTag {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "FungusFighter")]
    FungusFighter,

    #[serde(rename = "Hdj")]
    Hdj,

    #[serde(rename = "Hdj02")]
    Hdj02,

    #[serde(rename = "Irodori")]
    Irodori,

    #[serde(rename = "Vintage")]
    Vintage,
}

pub fn load() -> Result<NewActivityExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "NewActivityExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
