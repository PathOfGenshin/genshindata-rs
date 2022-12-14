// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type FungusExcelConfigData = Vec<FungusExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FungusExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "AAMAGGPFIHM")]
    pub aamaggpfihm: i64,

    #[serde(rename = "HDOIHBPMDIB")]
    pub hdoihbpmdib: i64,

    #[serde(rename = "EGEKEBGLKEP")]
    pub egekebglkep: i64,

    #[serde(rename = "COHLGCFKJJC")]
    pub cohlgcfkjjc: Option<i64>,

    #[serde(rename = "PKDDLMPIFNL")]
    pub pkddlmpifnl: Vec<i64>,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "JLEELAEIILI")]
    pub jleelaeiili: f64,

    #[serde(rename = "ADLIOOELDMF")]
    pub adliooeldmf: String,

    #[serde(rename = "HMOKEIHLAGD")]
    pub hmokeihlagd: String,

    #[serde(rename = "ALGKKLOHKHO")]
    pub algkklohkho: String,

    #[serde(rename = "MOJJMGOPJAD")]
    pub mojjmgopjad: String,

    #[serde(rename = "HHCHKLCFGKG")]
    pub hhchklcfgkg: String,

    #[serde(rename = "OEJBCLOPDHL")]
    pub oejbclopdhl: i64,

    #[serde(rename = "JEIBKMABGNL")]
    pub jeibkmabgnl: Vec<i64>,

    #[serde(rename = "MNFMACACEAE")]
    pub mnfmacaceae: Vec<i64>,

    #[serde(rename = "infoDescTextMapHash")]
    pub info_desc_text_map_hash: i64,

    #[serde(rename = "DOFNHNEGKGI")]
    pub dofnhnegkgi: i64,

    #[serde(rename = "DIPJIHDDIFH")]
    pub dipjihddifh: i64,
}

pub fn load() -> Result<FungusExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FungusExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
