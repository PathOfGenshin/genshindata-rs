/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LoadingSituationExcelConfigData = Vec<LoadingSituationExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LoadingSituationExcelConfigDatum {
    #[serde(rename = "stageID")]
    pub stage_id: i64,
    pub eehglbncpek: String,
    pub kdcnniggohp: Vec<i64>,
    pub fehepgbpdph: Vec<i64>,
    pub ioakcdpichn: Option<Ioakcdpichn>,
    #[serde(rename = "picPath")]
    pub pic_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Ioakcdpichn {
    #[serde(rename = "LOADING_AREA_CITY")]
    LoadingAreaCity,
    #[serde(rename = "LOADING_AREA_OUTDOOR")]
    LoadingAreaOutdoor,
}
