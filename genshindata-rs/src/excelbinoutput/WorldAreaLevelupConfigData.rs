// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type WorldAreaLevelupConfigData = Vec<WorldAreaLevelupConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldAreaLevelupConfigDatum {
    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "areaId")]
    pub area_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "consumeItem")]
    pub consume_item: ConsumeItem,

    #[serde(rename = "actionVec")]
    pub action_vec: Vec<ActionVec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionVec {
    #[serde(rename = "param1Vec")]
    pub param1_vec: Vec<i64>,

    #[serde(rename = "param2Vec")]
    pub param2_vec: Vec<i64>,

    #[serde(rename = "type")]
    pub action_vec_type: Option<Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumeItem {
    #[serde(rename = "itemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "itemNum")]
    pub item_num: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "WORLD_AREA_ACTION_ACTIVATE_ITEM")]
    WorldAreaActionActivateItem,

    #[serde(rename = "WORLD_AREA_ACTION_IMPROVE_STAMINA")]
    WorldAreaActionImproveStamina,

    #[serde(rename = "WORLD_AREA_ACTION_UNLOCK_FORCE")]
    WorldAreaActionUnlockForce,
}

pub fn load() -> Result<WorldAreaLevelupConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "WorldAreaLevelupConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
