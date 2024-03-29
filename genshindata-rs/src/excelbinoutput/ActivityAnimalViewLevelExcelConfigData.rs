/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityAnimalViewLevelExcelConfigData = Vec<ActivityAnimalViewLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityAnimalViewLevelExcelConfigDatum {
    pub id: i64,
    pub gallery_id: i64,
    #[serde(rename = "KMAIBHFMEHG")]
    pub kmaibhfmehg: Vec<i64>,
    #[serde(rename = "HKPAFMCANHB")]
    pub hkpafmcanhb: i64,
    pub group_link_id: i64,
    #[serde(rename = "IFBJLFKDPMP")]
    pub ifbjlfkdpmp: i64,
    #[serde(rename = "AOOAMLHLAOP")]
    pub aooamlhlaop: i64,
    pub open_day: i64,
    pub watcher_id: i64,
    pub level_title_text_map_hash: i64,
    #[serde(rename = "FBBCGANIBFH")]
    pub fbbcganibfh: i64,
}
