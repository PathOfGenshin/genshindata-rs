// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgDeckFieldExcelConfigData = Vec<GcgDeckFieldExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgDeckFieldExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KEMBEMMIJNB")]
    pub kembemmijnb: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "JFDMGHDNJCM")]
    pub jfdmghdnjcm: i64,

    #[serde(rename = "KACLCHCDMHP")]
    pub kaclchcdmhp: String,

    #[serde(rename = "DPIFMABGBBI")]
    pub dpifmabgbbi: i64,

    #[serde(rename = "AFPJOPOPDJH")]
    pub afpjopopdjh: i64,

    #[serde(rename = "OONJDOCLJMI")]
    pub oonjdocljmi: Vec<String>,

    #[serde(rename = "KHAMBKOLPDP")]
    pub khambkolpdp: Vec<String>,

    #[serde(rename = "PDEKMLCGADM")]
    pub pdekmlcgadm: Vec<String>,

    #[serde(rename = "FPEEHLBABFJ")]
    pub fpeehlbabfj: String,

    #[serde(rename = "IKMBALDBBII")]
    pub ikmbaldbbii: String,

    #[serde(rename = "NGJCFCECFLD")]
    pub ngjcfcecfld: String,

    #[serde(rename = "id")]
    pub id: Option<i64>,
}

pub fn load() -> Result<GcgDeckFieldExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGDeckFieldExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
