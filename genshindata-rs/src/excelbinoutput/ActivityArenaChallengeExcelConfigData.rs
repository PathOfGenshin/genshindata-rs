/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityArenaChallengeExcelConfigData = Vec<ActivityArenaChallengeExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityArenaChallengeExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,
    pub schedule_id: i64,
    pub arena_challenge_id: i64,
    pub arena_challenge_level: i64,
    pub watcher_id: i64,
    pub challenge_target_text_map_hash: i64,
    pub challenge_target1_text_map_hash: i64,
    pub challenge_target2_text_map_hash: i64,
    #[serde(rename = "LNICJFJKDAK")]
    pub lnicjfjkdak: Lnicjfjkdak,
    #[serde(rename = "BMJIOALJOAK")]
    pub bmjioaljoak: String,
    #[serde(rename = "AJDDBKGPHML")]
    pub ajddbkgphml: Ajddbkgphml,
    #[serde(rename = "BCNBCPCOFLG")]
    pub bcnbcpcoflg: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ajddbkgphml {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "2010035;1")]
    The20100351,
    #[serde(rename = "2010035;12")]
    The201003512,
    #[serde(rename = "2010035;2")]
    The20100352,
    #[serde(rename = "2010035;3")]
    The20100353,
    #[serde(rename = "2010035;4")]
    The20100354,
    #[serde(rename = "2010035;6")]
    The20100356,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Lnicjfjkdak {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "2010036;180")]
    The2010036180,
    #[serde(rename = "2010069;150")]
    The2010069150,
    #[serde(rename = "4002003;180")]
    The4002003180,
    #[serde(rename = "4002004;150")]
    The4002004150,
}
