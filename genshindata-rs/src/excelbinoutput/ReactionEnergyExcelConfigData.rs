// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ReactionEnergyExcelConfigData = Vec<ReactionEnergyExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionEnergyExcelConfigDatum {
    #[serde(rename = "type")]
    pub reaction_energy_excel_config_datum_type: String,

    #[serde(rename = "minDurability")]
    pub min_durability: f64,

    #[serde(rename = "maxDurability")]
    pub max_durability: f64,

    #[serde(rename = "costRatio")]
    pub cost_ratio: f64,

    #[serde(rename = "configID")]
    pub config_id: i64,

    #[serde(rename = "poolSize")]
    pub pool_size: f64,

    #[serde(rename = "poolRevivePeriod")]
    pub pool_revive_period: f64,

    #[serde(rename = "poolReviveEnergy")]
    pub pool_revive_energy: f64,

    #[serde(rename = "isPersistent")]
    pub is_persistent: Option<bool>,

    #[serde(rename = "costPeriod")]
    pub cost_period: Option<f64>,

    #[serde(rename = "dropProb")]
    pub drop_prob: i64,
}

pub fn load() -> Result<ReactionEnergyExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ReactionEnergyExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
