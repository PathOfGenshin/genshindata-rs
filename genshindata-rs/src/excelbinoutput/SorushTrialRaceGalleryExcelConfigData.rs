/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SorushTrialRaceGalleryExcelConfigData = Vec<SorushTrialRaceGalleryExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SorushTrialRaceGalleryExcelConfigDatum {
    pub id: i64,
    pub gallery_id: i64,
    #[serde(rename = "GELFLJLKCAK")]
    pub gelfljlkcak: i64,
    #[serde(rename = "CLAMIMGKEBJ")]
    pub clamimgkebj: i64,
    pub level_title_text_map_hash: i64,
    pub level_desc_text_map_hash: i64,
    #[serde(rename = "groupLinkID")]
    pub group_link_id: i64,
    #[serde(rename = "groupID")]
    pub group_id: i64,
    pub watcher_list: Vec<i64>,
}
