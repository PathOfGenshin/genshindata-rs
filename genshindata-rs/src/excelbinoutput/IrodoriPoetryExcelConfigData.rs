// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriPoetryExcelConfigData = Vec<IrodoriPoetryExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriPoetryExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "KMICIHENLHB")]
    pub kmicihenlhb: i64,

    #[serde(rename = "LABNOEGPADP")]
    pub labnoegpadp: i64,

    #[serde(rename = "entityType")]
    pub entity_type: String,

    #[serde(rename = "CDOGNDBLFCI")]
    pub cdogndblfci: Vec<Cdogndblfci>,

    #[serde(rename = "LBFNMFAMMKP")]
    pub lbfnmfammkp: i64,

    #[serde(rename = "EIJGEBDHMHK")]
    pub eijgebdhmhk: i64,

    #[serde(rename = "KEEJLEDDMDD")]
    pub keejleddmdd: i64,

    #[serde(rename = "JIOHGNDFONA")]
    pub jiohgndfona: Vec<i64>,

    #[serde(rename = "watcherID")]
    pub watcher_id: i64,

    #[serde(rename = "ODJNMNAIAOG")]
    pub odjnmnaiaog: i64,

    #[serde(rename = "CIDBMFPGJPE")]
    pub cidbmfpgjpe: i64,

    #[serde(rename = "ABKIJHCOMEO")]
    pub abkijhcomeo: i64,

    #[serde(rename = "GOENBADABLK")]
    pub goenbadablk: i64,

    #[serde(rename = "DFFOOGGMBDJ")]
    pub dffooggmbdj: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cdogndblfci {
    #[serde(rename = "BFGIHKGEBEK")]
    pub bfgihkgebek: Vec<i64>,

    #[serde(rename = "AFCKKKFPGBH")]
    pub afckkkfpgbh: Option<i64>,
}

pub fn load() -> Result<IrodoriPoetryExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "IrodoriPoetryExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
