// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ChannellerSlabBuffExcelConfigData = Vec<ChannellerSlabBuffExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannellerSlabBuffExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "buffNameTextMapHash")]
    pub buff_name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParam")]
    pub desc_param: Vec<String>,

    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "buffQualityType")]
    pub buff_quality_type: BuffQualityType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuffQualityType {
    #[serde(rename = "QUALITY_BLUE")]
    QualityBlue,

    #[serde(rename = "QUALITY_GREEN")]
    QualityGreen,

    #[serde(rename = "QUALITY_PURPLE")]
    QualityPurple,
}

pub fn load() -> Result<ChannellerSlabBuffExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ChannellerSlabBuffExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
