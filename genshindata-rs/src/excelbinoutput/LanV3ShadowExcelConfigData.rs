// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LanV3ShadowExcelConfigData = Vec<LanV3ShadowExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanV3ShadowExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "ABNKCOAELEN")]
    pub abnkcoaelen: f64,

    #[serde(rename = "GFKJJAELCNF")]
    pub gfkjjaelcnf: f64,

    #[serde(rename = "ELJPILANMFI")]
    pub eljpilanmfi: f64,

    #[serde(rename = "EKCNIAICABI")]
    pub ekcniaicabi: f64,

    #[serde(rename = "MLKLOHCAGEP")]
    pub mlklohcagep: f64,

    #[serde(rename = "LIDJGIDOGFB")]
    pub lidjgidogfb: i64,

    #[serde(rename = "NPJCBPNFNFK")]
    pub npjcbpnfnfk: i64,

    #[serde(rename = "BACPOACGFCA")]
    pub bacpoacgfca: i64,

    #[serde(rename = "NJHLPEIIGBM")]
    pub njhlpeiigbm: Vec<f64>,

    #[serde(rename = "FLBFLCBLKPJ")]
    pub flbflcblkpj: f64,

    #[serde(rename = "OLGHFGHOKBM")]
    pub olghfghokbm: i64,

    #[serde(rename = "LMGBKMHFFOK")]
    pub lmgbkmhffok: i64,

    #[serde(rename = "PMNMBBLJFPA")]
    pub pmnmbbljfpa: i64,
}

pub fn load() -> Result<LanV3ShadowExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LanV3ShadowExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}