// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type EmbeddedTextMapConfigData = Vec<EmbeddedTextMapConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddedTextMapConfigDatum {
    #[serde(rename = "textMapId")]
    pub text_map_id: String,

    #[serde(rename = "textMapContentTextMapHash")]
    pub text_map_content_text_map_hash: i64,
}

pub fn load() -> Result<EmbeddedTextMapConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "EmbeddedTextMapConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
