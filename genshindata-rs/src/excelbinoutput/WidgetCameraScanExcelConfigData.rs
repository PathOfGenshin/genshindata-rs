// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type WidgetCameraScanExcelConfigData = Vec<WidgetCameraScanExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WidgetCameraScanExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cameraID")]
    pub camera_id: i64,

    #[serde(rename = "configID")]
    pub config_id: i64,

    #[serde(rename = "CBBIHAGHMDI")]
    pub cbbihaghmdi: Vec<i64>,

    #[serde(rename = "BCOBBPEMKBB")]
    pub bcobbpemkbb: bool,

    #[serde(rename = "EBAAEONKGGO")]
    pub ebaaeonkggo: String,
}

pub fn load() -> Result<WidgetCameraScanExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "WidgetCameraScanExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
