/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type QteExcelConfigData = Vec<QteExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QteExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub pejkeboekgp: i64,
    pub dmocaceedcb: String,
    pub pojkgdldebe: Vec<Hgckpdnocmo>,
    pub mpkocmoodkl: Vec<Mpkocmoodkl>,
    pub hgckpdnocmo: Vec<Hgckpdnocmo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hgckpdnocmo {
    pub param: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mpkocmoodkl {
    #[serde(rename = "type")]
    pub mpkocmoodkl_type: Option<String>,
    pub param: Vec<i64>,
}
