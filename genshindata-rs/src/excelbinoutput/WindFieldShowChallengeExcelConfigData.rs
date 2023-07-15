/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WindFieldShowChallengeExcelConfigData = Vec<WindFieldShowChallengeExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct WindFieldShowChallengeExcelConfigDatum {
    pub mjdchomjlbj: i64,
    pub ampoaflmmlo: Option<i64>,
    #[serde(rename = "showType")]
    pub show_type: String,
    pub dgpljfbgdij: Vec<i64>,
    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,
    pub pegeldeenpm: i64,
    pub anlplekhgbd: i64,
    pub aljdjleiipp: Option<i64>,
}
