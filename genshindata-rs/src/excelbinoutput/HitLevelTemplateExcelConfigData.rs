// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HitLevelTemplateExcelConfigData = Vec<HitLevelTemplateExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HitLevelTemplateExcelConfigDatum {
    #[serde(rename = "type")]
    pub hit_level_template_excel_config_datum_type: String,

    #[serde(rename = "hitLevel")]
    pub hit_level: String,

    #[serde(rename = "hitImpulseX")]
    pub hit_impulse_x: Option<f64>,

    #[serde(rename = "hitImpulseY")]
    pub hit_impulse_y: Option<f64>,
}

pub fn load() -> Result<HitLevelTemplateExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HitLevelTemplateExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
