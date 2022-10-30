// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;
use std::collections::HashMap;

pub type IrodoriChessMapExcelConfigData = Vec<IrodoriChessMapExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriChessMapExcelConfigDatum {
    #[serde(rename = "MFBLPEKIBJL")]
    pub mfblpekibjl: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "CIJIOEACOEF")]
    pub cijioeacoef: i64,

    #[serde(rename = "show")]
    pub show: bool,

    #[serde(rename = "KJMGFIOGBEE")]
    pub kjmgfiogbee: Vec<i64>,

    #[serde(rename = "JJEIACBLBGB")]
    pub jjeiacblbgb: i64,

    #[serde(rename = "NHIIADEADCD")]
    pub nhiiadeadcd: HashMap<String, i64>,

    #[serde(rename = "HDFAPHAIOKJ")]
    pub hdfaphaiokj: Vec<i64>,

    #[serde(rename = "MJKOLFFBELA")]
    pub mjkolffbela: Vec<i64>,

    #[serde(rename = "JMEGDOMIJDG")]
    pub jmegdomijdg: Vec<i64>,

    #[serde(rename = "KBNCBFEKOCE")]
    pub kbncbfekoce: Vec<i64>,

    #[serde(rename = "OOMNKDOGGNL")]
    pub oomnkdoggnl: i64,

    #[serde(rename = "mapNameTextMapHash")]
    pub map_name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "mapIconPath")]
    pub map_icon_path: String,

    #[serde(rename = "EGKIJELJKPD")]
    pub egkijeljkpd: HashMap<String, i64>,

    #[serde(rename = "MNJLIBEEPNM")]
    pub mnjlibeepnm: Option<i64>,
}

pub fn load() -> Result<IrodoriChessMapExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "IrodoriChessMapExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
