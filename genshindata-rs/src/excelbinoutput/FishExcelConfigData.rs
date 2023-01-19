// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type FishExcelConfigData = Vec<FishExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FishExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "monsterId")]
    pub monster_id: i64,

    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "hp")]
    pub hp: i64,

    #[serde(rename = "FOAKGCHNLGD")]
    pub foakgchnlgd: Vec<i64>,

    #[serde(rename = "FCOJFGDHFJG")]
    pub fcojfgdhfjg: i64,

    #[serde(rename = "AMAHALAHNJD")]
    pub amahalahnjd: i64,

    #[serde(rename = "OAJMLKLMLPG")]
    pub oajmlklmlpg: Vec<f64>,

    #[serde(rename = "OCGJNOEBECL")]
    pub ocgjnoebecl: Vec<i64>,

    #[serde(rename = "KINMFOCBDHP")]
    pub kinmfocbdhp: Vec<f64>,

    #[serde(rename = "DOMGJBAENPH")]
    pub domgjbaenph: Vec<f64>,

    #[serde(rename = "LGHMMPJKKIN")]
    pub lghmmpjkkin: f64,

    #[serde(rename = "DAEIBIDGCMK")]
    pub daeibidgcmk: f64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "PACMBOOHLNP")]
    pub pacmboohlnp: Vec<Option<serde_json::Value>>,

    #[serde(rename = "NPFFNLGANOL")]
    pub npffnlganol: i64,

    #[serde(rename = "AEGOHIJAHJE")]
    pub aegohijahje: i64,

    #[serde(rename = "EMNMMPBNGBN")]
    pub emnmmpbngbn: Option<i64>,

    #[serde(rename = "BHKCEIBBGKL")]
    pub bhkceibbgkl: Option<i64>,
}

pub fn load() -> Result<FishExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FishExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
