/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SorushTrialPhotoMatchGalleryExcelConfigData = Vec<SorushTrialPhotoMatchGalleryExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SorushTrialPhotoMatchGalleryExcelConfigDatum {
    pub ljgnogokmcf: Vec<f64>,
    pub hgpbfkcbibd: Vec<f64>,
    pub himdiokbakk: i64,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "galleryId")]
    pub gallery_id: i64,
    pub dehhfkbjiic: i64,
    pub efebbibcahk: i64,
    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,
    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,
    pub pggijcdncam: i64,
    #[serde(rename = "groupID")]
    pub group_id: i64,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
}
