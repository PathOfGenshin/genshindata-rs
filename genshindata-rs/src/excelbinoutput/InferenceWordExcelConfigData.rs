// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type InferenceWordExcelConfigData = Vec<InferenceWordExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InferenceWordExcelConfigDatum {
    #[serde(rename = "ECIMFDJCFBG")]
    pub ecimfdjcfbg: i64,

    #[serde(rename = "INPDDAGAPKG")]
    pub inpddagapkg: i64,

    #[serde(rename = "OPJEPFFMDNF")]
    pub opjepffmdnf: Option<bool>,

    #[serde(rename = "HBGMJCICPGP")]
    pub hbgmjcicpgp: Option<bool>,

    #[serde(rename = "ICODKFPNCPB")]
    pub icodkfpncpb: Vec<Aeocagefjmp>,

    #[serde(rename = "IKKEHHCMOIJ")]
    pub ikkehhcmoij: Option<bool>,

    #[serde(rename = "JJGEEJCFAOH")]
    pub jjgeejcfaoh: Option<i64>,

    #[serde(rename = "AEOCAGEFJMP")]
    pub aeocagefjmp: Vec<Aeocagefjmp>,

    #[serde(rename = "MLCAIFCCEKP")]
    pub mlcaifccekp: i64,

    #[serde(rename = "BNMHOMKCLHA")]
    pub bnmhomkclha: Option<i64>,

    #[serde(rename = "LPKOLDHFCND")]
    pub lpkoldhfcnd: Option<i64>,

    #[serde(rename = "OKNFGNMMIGD")]
    pub oknfgnmmigd: Option<i64>,

    #[serde(rename = "MMHBLHFKLPM")]
    pub mmhblhfklpm: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aeocagefjmp {
    #[serde(rename = "type")]
    pub aeocagefjmp_type: Option<String>,

    #[serde(rename = "param")]
    pub param: String,
}

pub fn load() -> Result<InferenceWordExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "InferenceWordExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
