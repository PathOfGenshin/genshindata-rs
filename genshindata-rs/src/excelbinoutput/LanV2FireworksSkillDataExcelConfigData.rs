// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type LanV2FireworksSkillDataExcelConfigData = Vec<LanV2FireworksSkillDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanV2FireworksSkillDataExcelConfigDatum {
    #[serde(rename = "IKNKIEBAPOG")]
    pub iknkiebapog: i64,

    #[serde(rename = "MMGDOOFIPMG")]
    pub mmgdoofipmg: String,

    #[serde(rename = "NONBGKJFAEF")]
    pub nonbgkjfaef: i64,

    #[serde(rename = "LMEJOCNOECB")]
    pub lmejocnoecb: i64,

    #[serde(rename = "BFBIPOBAMMH")]
    pub bfbipobammh: Option<i64>,

    #[serde(rename = "DGAIEGOKIKP")]
    pub dgaiegokikp: Option<i64>,

    #[serde(rename = "NCOMAFNKJNK")]
    pub ncomafnkjnk: Option<i64>,

    #[serde(rename = "HMHNMKBMFNO")]
    pub hmhnmkbmfno: Vec<i64>,

    #[serde(rename = "EIMJFLKOFPK")]
    pub eimjflkofpk: i64,

    #[serde(rename = "DCHPDLPJDMO")]
    pub dchpdlpjdmo: i64,

    #[serde(rename = "HJOICJDIEAE")]
    pub hjoicjdieae: Vec<i64>,

    #[serde(rename = "ONFMNCPFFJI")]
    pub onfmncpffji: Option<i64>,
}

pub fn load() -> Result<LanV2FireworksSkillDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "LanV2FireworksSkillDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
