/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityCharAmusementLevelExcelConfigData = Vec<ActivityCharAmusementLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityCharAmusementLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,
    #[serde(rename = "weight")]
    pub weight: i64,
    pub amidlbbfihh: Amidlbbfihh,
    #[serde(rename = "galleryId")]
    pub gallery_id: i64,
    pub penfodfehao: i64,
    pub ggmmfcnepce: i64,
    pub okaomegbhbi: Vec<i64>,
    pub mflpieodicp: String,
    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,
    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,
    pub ppphgocafna: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Amidlbbfihh {
    #[serde(rename = "LEVEL_TYPE_AMUSEMENT")]
    LevelTypeAmusement,
    #[serde(rename = "LEVEL_TYPE_FIGHT")]
    LevelTypeFight,
}
