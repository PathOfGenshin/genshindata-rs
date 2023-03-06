// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

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

    #[serde(rename = "JAEBCLDFHGA")]
    pub jaebcldfhga: Vec<i64>,

    #[serde(rename = "IJLHBJGHEID")]
    pub ijlhbjgheid: i64,

    #[serde(rename = "CDJENEDEBGG")]
    pub cdjenedebgg: i64,

    #[serde(rename = "ELPDCDELBAH")]
    pub elpdcdelbah: Vec<f64>,

    #[serde(rename = "BLDDIOHDFAM")]
    pub blddiohdfam: Vec<i64>,

    #[serde(rename = "JHMEAEJHPFI")]
    pub jhmeaejhpfi: Vec<f64>,

    #[serde(rename = "LHOBHBJEGMC")]
    pub lhobhbjegmc: Vec<f64>,

    #[serde(rename = "NHKFMCEKEGA")]
    pub nhkfmcekega: f64,

    #[serde(rename = "OOEFNHENLJG")]
    pub ooefnhenljg: f64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "KCNOHFIHICM")]
    pub kcnohfihicm: Vec<Option<serde_json::Value>>,

    #[serde(rename = "HKHJPHKBODA")]
    pub hkhjphkboda: i64,

    #[serde(rename = "PBCLNPKDAJD")]
    pub pbclnpkdajd: i64,

    #[serde(rename = "FLCIDEFOMIB")]
    pub flcidefomib: Option<i64>,

    #[serde(rename = "IHKADDNCAPD")]
    pub ihkaddncapd: Option<i64>,
}

pub fn load() -> Result<FishExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FishExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
