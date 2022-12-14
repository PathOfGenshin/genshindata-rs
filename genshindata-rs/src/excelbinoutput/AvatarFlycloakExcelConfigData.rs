// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type AvatarFlycloakExcelConfigData = Vec<AvatarFlycloakExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarFlycloakExcelConfigDatum {
    #[serde(rename = "flycloakId")]
    pub flycloak_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "prefabPath")]
    pub prefab_path: String,

    #[serde(rename = "jsonName")]
    pub json_name: JsonName,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JsonName {
    #[serde(rename = "Flycloak_Default_01")]
    FlycloakDefault01,
}

pub fn load() -> Result<AvatarFlycloakExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "AvatarFlycloakExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
