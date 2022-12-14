// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type IrodoriChessGearExcelConfigData = Vec<IrodoriChessGearExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriChessGearExcelConfigDatum {
    #[serde(rename = "DAPCKMONPPB")]
    pub dapckmonppb: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "JLMNLNPBEKP")]
    pub jlmnlnpbekp: i64,

    #[serde(rename = "gearNameTextMapHash")]
    pub gear_name_text_map_hash: i64,

    #[serde(rename = "gearShortNameTextMapHash")]
    pub gear_short_name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "gearIconPath")]
    pub gear_icon_path: String,

    #[serde(rename = "JPIGAHDGKHJ")]
    pub jpigahdgkhj: String,

    #[serde(rename = "mapIconPath")]
    pub map_icon_path: String,

    #[serde(rename = "attack")]
    pub attack: i64,

    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,

    #[serde(rename = "attackRange")]
    pub attack_range: i64,

    #[serde(rename = "isEnableRotate")]
    pub is_enable_rotate: Option<bool>,

    #[serde(rename = "KCKGHOGJOBM")]
    pub kckghogjobm: Option<i64>,
}

pub fn load() -> Result<IrodoriChessGearExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "IrodoriChessGearExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
