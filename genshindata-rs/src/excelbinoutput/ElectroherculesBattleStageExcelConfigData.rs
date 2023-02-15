// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ElectroherculesBattleStageExcelConfigData = Vec<ElectroherculesBattleStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectroherculesBattleStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "NCHJIOAEMEA")]
    pub nchjioaemea: i64,

    #[serde(rename = "KHAKJEOMIAC")]
    pub khakjeomiac: Vec<i64>,

    #[serde(rename = "DOHKEDMCCHK")]
    pub dohkedmcchk: i64,

    #[serde(rename = "NOLJOMCJGIB")]
    pub noljomcjgib: i64,

    #[serde(rename = "EKDFABPEMLI")]
    pub ekdfabpemli: i64,

    #[serde(rename = "HFPMGBHIEBG")]
    pub hfpmgbhiebg: String,

    #[serde(rename = "GEELAMBOKAC")]
    pub geelambokac: String,

    #[serde(rename = "MBMHENNOPNH")]
    pub mbmhennopnh: String,

    #[serde(rename = "ICNDAMONABN")]
    pub icndamonabn: String,

    #[serde(rename = "KDHLCBPEFKA")]
    pub kdhlcbpefka: i64,

    #[serde(rename = "EPBPJPOGPPC")]
    pub epbpjpogppc: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: Option<i64>,

    #[serde(rename = "NOIEMHJHNAL")]
    pub noiemhjhnal: Option<i64>,
}

pub fn load() -> Result<ElectroherculesBattleStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ElectroherculesBattleStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
