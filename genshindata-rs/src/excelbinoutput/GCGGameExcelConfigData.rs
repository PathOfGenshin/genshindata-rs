// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgGameExcelConfigData = Vec<GcgGameExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgGameExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MBOLNEAALBF")]
    pub mbolneaalbf: i64,

    #[serde(rename = "MCEDOPGBHPM")]
    pub mcedopgbhpm: i64,

    #[serde(rename = "MIJHPKBIOPI")]
    pub mijhpkbiopi: Option<i64>,

    #[serde(rename = "GHDGOBAAJGM")]
    pub ghdgobaajgm: Option<i64>,

    #[serde(rename = "guideName")]
    pub guide_name: GuideName,

    #[serde(rename = "DNMCOPPPNLB")]
    pub dnmcopppnlb: Option<Dnmcopppnlb>,

    #[serde(rename = "GLMOGBIKNFG")]
    pub glmogbiknfg: Option<Glmogbiknfg>,

    #[serde(rename = "AFDLJEHJMBM")]
    pub afdljehjmbm: Option<bool>,

    #[serde(rename = "DOIPEHEKFHD")]
    pub doipehekfhd: Option<i64>,

    #[serde(rename = "EOKDDNCPBJF")]
    pub eokddncpbjf: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Dnmcopppnlb {
    #[serde(rename = "AI")]
    Ai,

    #[serde(rename = "PVE")]
    Pve,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Glmogbiknfg {
    #[serde(rename = "SELF")]
    GlmogbiknfgSelf,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GuideName {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Tutorial_1_1")]
    Tutorial1_1,

    #[serde(rename = "Tutorial_1_2")]
    Tutorial1_2,

    #[serde(rename = "Tutorial_1_4")]
    Tutorial1_4,

    #[serde(rename = "Tutorial_1_5")]
    Tutorial1_5,

    #[serde(rename = "Tutorial_2_1")]
    Tutorial2_1,

    #[serde(rename = "Tutorial_2_2")]
    Tutorial2_2,
}

pub fn load() -> Result<GcgGameExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGGameExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
