/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySpiceStageDataExcelConfigData = Vec<ActivitySpiceStageDataExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivitySpiceStageDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub ofmpckbpkpi: i64,
    pub lfaifkfemfb: i64,
    pub kpbcnfcolad: Vec<i64>,
    pub hpeghjkjofi: Vec<i64>,
    pub pcjpifilcme: Vec<i64>,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
    pub nofdpfhmfnm: i64,
    pub gdpdigodlpi: i64,
    pub peedccnpjod: i64,
    pub fhkfpgbnahp: Vec<i64>,
    #[serde(rename = "watcherId")]
    pub watcher_id: i64,
}
