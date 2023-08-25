/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BartenderFormulaExcelConfigData = Vec<BartenderFormulaExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BartenderFormulaExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub lbjhmmamcoh: Vec<Cflopncegni>,
    pub cflopncegni: Vec<Cflopncegni>,
    pub fpkaaeechla: Vec<i64>,
    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    pub kjmdnpfihgc: Option<i64>,
    pub lpphpgajocl: Option<i64>,
    pub imcjbladaka: i64,
    pub gmodnnlaolf: Option<Gmodnnlaolf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cflopncegni {
    pub id: Option<i64>,
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gmodnnlaolf {
    #[serde(rename = "HEAVY")]
    Heavy,
    #[serde(rename = "MIDDLE")]
    Middle,
}
