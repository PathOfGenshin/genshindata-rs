// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type DocumentExcelConfigData = Vec<DocumentExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentLocalizedId")]
    pub content_localized_id: Option<i64>,

    #[serde(rename = "previewPath")]
    pub preview_path: String,

    #[serde(rename = "videoPath")]
    pub video_path: VideoPath,

    #[serde(rename = "PMDFHOFBMAF")]
    pub pmdfhofbmaf: Vec<i64>,

    #[serde(rename = "OPAHFFDFJIE")]
    pub opahffdfjie: Vec<i64>,

    #[serde(rename = "documentType")]
    pub document_type: Option<String>,

    #[serde(rename = "subtitleID")]
    pub subtitle_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VideoPath {
    #[serde(rename = "Ambor_Readings.usm")]
    AmborReadingsUsm,

    #[serde(rename = "")]
    Empty,
}

pub fn load() -> Result<DocumentExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DocumentExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
