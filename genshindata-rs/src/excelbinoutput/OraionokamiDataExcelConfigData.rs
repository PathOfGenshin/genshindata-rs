// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type OraionokamiDataExcelConfigData = Vec<OraionokamiDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OraionokamiDataExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "MCAGNOJALOE")]
    pub mcagnojaloe: i64,

    #[serde(rename = "JAFLCPODJLA")]
    pub jaflcpodjla: i64,

    #[serde(rename = "serverBuffId")]
    pub server_buff_id: i64,

    #[serde(rename = "HHEFLAJCCCL")]
    pub hheflajcccl: Vec<i64>,

    #[serde(rename = "PPGBLPGAPBE")]
    pub ppgblpgapbe: i64,

    #[serde(rename = "FNCOIILOKJO")]
    pub fncoiilokjo: i64,

    #[serde(rename = "HHHAEKBJKCL")]
    pub hhhaekbjkcl: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "MLIPIBGFLIK")]
    pub mlipibgflik: i64,

    #[serde(rename = "iconPath")]
    pub icon_path: String,
}

pub fn load() -> Result<OraionokamiDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "OraionokamiDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
