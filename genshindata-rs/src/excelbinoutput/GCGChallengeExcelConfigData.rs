/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgChallengeExcelConfigData = Vec<GcgChallengeExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcgChallengeExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "type")]
    pub gcg_challenge_excel_config_datum_type: String,
    pub param_list: Vec<String>,
    #[serde(rename = "MKLMKDKBBNL")]
    pub mklmkdkbbnl: i64,
    pub progress: i64,
    #[serde(rename = "FNILAAGCLON")]
    pub fnilaagclon: Option<bool>,
}
