// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ClientSceneTagConfigData = Vec<ClientSceneTagConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSceneTagConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sceneTagName")]
    pub scene_tag_name: String,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "NECJFGPMKHG")]
    pub necjfgpmkhg: String,
}

pub fn load() -> Result<ClientSceneTagConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ClientSceneTagConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
