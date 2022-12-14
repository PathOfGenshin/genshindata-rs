// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgRuleExcelConfigData = Vec<GcgRuleExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgRuleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "AKHAMLGIGLC")]
    pub akhamlgiglc: i64,

    #[serde(rename = "LCOEKIEHODB")]
    pub lcoekiehodb: i64,

    #[serde(rename = "BNMGFLEIHCG")]
    pub bnmgfleihcg: i64,

    #[serde(rename = "ACIJLHLKCHJ")]
    pub acijlhlkchj: Vec<i64>,

    #[serde(rename = "COAANBEJJCM")]
    pub coaanbejjcm: i64,

    #[serde(rename = "BFFADOHPEOA")]
    pub bffadohpeoa: Option<i64>,

    #[serde(rename = "PGOEDEPAHOF")]
    pub pgoedepahof: Option<i64>,

    #[serde(rename = "FKLPBOMJAMH")]
    pub fklpbomjamh: Option<i64>,

    #[serde(rename = "DFAKLOJHDFD")]
    pub dfaklojhdfd: Option<i64>,

    #[serde(rename = "AJOGNOBLHLH")]
    pub ajognoblhlh: Option<i64>,

    #[serde(rename = "OJGHEOAHNHF")]
    pub ojgheoahnhf: Option<i64>,

    #[serde(rename = "KMICDHGHBGP")]
    pub kmicdhghbgp: Option<i64>,

    #[serde(rename = "NFCJIKFIFKF")]
    pub nfcjikfifkf: Option<i64>,

    #[serde(rename = "JMFMIHAIACL")]
    pub jmfmihaiacl: Option<i64>,

    #[serde(rename = "GGGOFJCALKI")]
    pub gggofjcalki: Option<i64>,

    #[serde(rename = "BNOCJECLGJC")]
    pub bnocjeclgjc: Option<f64>,
}

pub fn load() -> Result<GcgRuleExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGRuleExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
