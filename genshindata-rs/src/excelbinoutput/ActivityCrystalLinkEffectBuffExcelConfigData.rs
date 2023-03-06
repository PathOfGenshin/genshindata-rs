// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityCrystalLinkEffectBuffExcelConfigData = Vec<ActivityCrystalLinkEffectBuffExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityCrystalLinkEffectBuffExcelConfigDatum {
    #[serde(rename = "buffId")]
    pub buff_id: i64,

    #[serde(rename = "LADKLKNNEKD")]
    pub ladklknnekd: String,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "KBPKBJGBILH")]
    pub kbpkbjgbilh: String,

    #[serde(rename = "HPEMLDAJDLJ")]
    pub hpemldajdlj: i64,

    #[serde(rename = "HMIMCBFCCIA")]
    pub hmimcbfccia: i64,

    #[serde(rename = "KKHHICCNGGI")]
    pub kkhhiccnggi: i64,

    #[serde(rename = "CCFPNGLDMCB")]
    pub ccfpngldmcb: Vec<String>,
}

pub fn load() -> Result<ActivityCrystalLinkEffectBuffExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityCrystalLinkEffectBuffExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
