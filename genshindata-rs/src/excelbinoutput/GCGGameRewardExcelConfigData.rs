/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgGameRewardExcelConfigData = Vec<GcgGameRewardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgGameRewardExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,
    pub iofmlpmemkl: Vec<Iofmlpmemkl>,
    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,
    pub hnklgbfplnd: i64,
    pub flichhcnchc: Vec<i64>,
    pub hmahmngembg: Vec<i64>,
    pub jeakbanfobp: Vec<i64>,
    #[serde(rename = "failTips")]
    pub fail_tips: Vec<i64>,
    #[serde(rename = "levelType")]
    pub level_type: Option<LevelType>,
    pub jmhbjakjcih: i64,
    pub fdhgfajjgjh: Option<String>,
    #[serde(rename = "condList")]
    pub cond_list: Vec<CondList>,
    pub cclpfmmddpg: Cclpfmmddpg,
    pub klmhnnomehl: Option<bool>,
    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,
    pub ebmfleaimkf: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cclpfmmddpg {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "Gcg_Loading_Bg2")]
    GcgLoadingBg2,
    #[serde(rename = "Gcg_Loading_Bg3")]
    GcgLoadingBg3,
    #[serde(rename = "Gcg_Loading_Bg4")]
    GcgLoadingBg4,
    #[serde(rename = "Gcg_Loading_Bg5")]
    GcgLoadingBg5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CondList {
    pub param_list: Vec<i64>,
    #[serde(rename = "type")]
    pub cond_list_type: Option<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "FINISH_LEVEL_CHALLENGE")]
    FinishLevelChallenge,
    #[serde(rename = "GCG_LEVEL")]
    GcgLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iofmlpmemkl {
    pub challenge_id: Option<i64>,
    pub reward_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LevelType {
    #[serde(rename = "PVE_MONSTER")]
    PveMonster,
}
