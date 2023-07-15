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
    #[serde(rename = "JHGAJBAHMNI")]
    pub jhgajbahmni: Jhgajbahmni,
    #[serde(rename = "FPJACEKKNLD")]
    pub fpjacekknld: Fpjacekknld,
    #[serde(rename = "HAMBCDBBBDH")]
    pub hambcdbbbdh: Hambcdbbbdh,
    #[serde(rename = "ABFPFLOMIEL")]
    pub abfpflomiel: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fpjacekknld {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "2010038;6")]
    The20100386,
    #[serde(rename = "2010039;12")]
    The201003912,
    #[serde(rename = "2010040;20")]
    The201004020,
    #[serde(rename = "2010041;15")]
    The201004115,
    #[serde(rename = "2010042;15")]
    The201004215,
    #[serde(rename = "2010043;20")]
    The201004320,
    #[serde(rename = "2010044;15")]
    The201004415,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Hambcdbbbdh {
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
pub enum Jhgajbahmni {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "2010036;180")]
    The2010036180,
    #[serde(rename = "2010069;150")]
    The2010069150,
}
