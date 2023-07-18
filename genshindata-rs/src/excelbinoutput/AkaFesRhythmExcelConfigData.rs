/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AkaFesRhythmExcelConfigData = Vec<AkaFesRhythmExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AkaFesRhythmExcelConfigDatum {
    pub id: i64,
    pub open_day: i64,
    pub level_name_text_map_hash: i64,
    pub score: i64,
    #[serde(rename = "CPFPCKBMIFK")]
    pub cpfpckbmifk: String,
    pub dungeon_id: i64,
    pub gallery_id: i64,
    #[serde(rename = "KEHGAKLKHKC")]
    pub kehgaklkhkc: i64,
    #[serde(rename = "CJGAAPFPEBA")]
    pub cjgaapfpeba: i64,
    pub watcher_id: Vec<i64>,
    pub tutorial_id: i64,
    #[serde(rename = "EDFENOCAEIM")]
    pub edfenocaeim: i64,
    #[serde(rename = "FOABAAPGMKA")]
    pub foabaapgmka: i64,
    #[serde(rename = "OMIIGKLBGGC")]
    pub omiigklbggc: Vec<i64>,
    #[serde(rename = "type")]
    pub aka_fes_rhythm_excel_config_datum_type: Option<String>,
}
