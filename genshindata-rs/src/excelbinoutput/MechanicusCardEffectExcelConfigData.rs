// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MechanicusCardEffectExcelConfigData = Vec<MechanicusCardEffectExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MechanicusCardEffectExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "targetType")]
    pub target_type: Option<TargetType>,

    #[serde(rename = "targetParamList")]
    pub target_param_list: Vec<i64>,

    #[serde(rename = "effectType")]
    pub effect_type: String,

    #[serde(rename = "effectStrParam")]
    pub effect_str_param: String,

    #[serde(rename = "effectParam1")]
    pub effect_param1: Option<i64>,

    #[serde(rename = "effectParam2")]
    pub effect_param2: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "MECHANICUS_CARD_TARGET_ALL")]
    MechanicusCardTargetAll,

    #[serde(rename = "MECHANICUS_CARD_TARGET_GADGETS")]
    MechanicusCardTargetGadgets,
}

pub fn load() -> Result<MechanicusCardEffectExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MechanicusCardEffectExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
