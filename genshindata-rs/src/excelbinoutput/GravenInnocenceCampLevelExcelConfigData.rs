/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GravenInnocenceCampLevelExcelConfigData = Vec<GravenInnocenceCampLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GravenInnocenceCampLevelExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "groupLinkID")]
    pub group_link_id: i64,
    pub challenge_index: i64,
    #[serde(rename = "WatcherID")]
    pub watcher_id: i64,
}
