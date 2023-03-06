// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ClientSceneTagConfigData = Vec<ClientSceneTagConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSceneTagConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sceneTagName")]
    pub scene_tag_name: String,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "BCFJBMODMLP")]
    pub bcfjbmodmlp: String,
}

pub fn load() -> Result<ClientSceneTagConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ClientSceneTagConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
