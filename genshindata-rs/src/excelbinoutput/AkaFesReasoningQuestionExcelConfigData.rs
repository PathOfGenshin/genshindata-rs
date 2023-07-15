/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AkaFesReasoningQuestionExcelConfigData = Vec<AkaFesReasoningQuestionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AkaFesReasoningQuestionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub hngjljmhhma: i64,
    pub bomdhonfgfc: Option<Bomdhonfgfc>,
    pub bhgnaigkdko: Vec<i64>,
    #[serde(rename = "sortId")]
    pub sort_id: i64,
    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
    pub megbpgfaeao: i64,
    pub fphoanlmdoh: i64,
    pub kmpkbpimpib: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bomdhonfgfc {
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "YES")]
    Yes,
}
