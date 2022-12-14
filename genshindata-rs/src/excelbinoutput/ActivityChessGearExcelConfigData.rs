// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityChessGearExcelConfigData = Vec<ActivityChessGearExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityChessGearExcelConfigDatum {
    #[serde(rename = "gearID")]
    pub gear_id: i64,

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

    #[serde(rename = "attack")]
    pub attack: i64,

    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,

    #[serde(rename = "attackRange")]
    pub attack_range: i64,

    #[serde(rename = "buildCost")]
    pub build_cost: i64,

    #[serde(rename = "demolitionRefund")]
    pub demolition_refund: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "PGNCEOMKDDK")]
    pub pgnceomkddk: Vec<i64>,

    #[serde(rename = "JLMNLNPBEKP")]
    pub jlmnlnpbekp: i64,

    #[serde(rename = "EDDKOPENING")]
    pub eddkopening: i64,

    #[serde(rename = "isEnableRotate")]
    pub is_enable_rotate: Option<bool>,

    #[serde(rename = "KCKGHOGJOBM")]
    pub kckghogjobm: Option<i64>,
}

pub fn load() -> Result<ActivityChessGearExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityChessGearExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
