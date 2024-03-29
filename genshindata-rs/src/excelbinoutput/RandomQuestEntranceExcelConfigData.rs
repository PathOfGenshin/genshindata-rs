/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RandomQuestEntranceExcelConfigData = Vec<RandomQuestEntranceExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomQuestEntranceExcelConfigDatum {
    pub id: i64,
    pub weight: i64,
    pub template_id: i64,
    pub filter_list: Vec<FilterList>,
    pub filter_logic_type: Option<FilterLogicType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FilterList {
    pub didegdkgonm: Didegdkgonm,
    pub nebmmpngpjg: Vec<i64>,
    pub njeplojhbce: Option<Njeplojhbce>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Didegdkgonm {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "{PlayerLevel}")]
    PlayerLevel,
    #[serde(rename = "{QuestNpcID}")]
    QuestNpcId,
    #[serde(rename = "{QuestNpcID2}")]
    QuestNpcId2,
    #[serde(rename = "{QuestRescuePos}")]
    QuestRescuePos,
    #[serde(rename = "{QuestRescuePos1}")]
    QuestRescuePos1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Njeplojhbce {
    #[serde(rename = "RQ_FILTER_NPC")]
    RqFilterNpc,
    #[serde(rename = "RQ_FILTER_PLAYER_LEVEL")]
    RqFilterPlayerLevel,
    #[serde(rename = "RQ_FILTER_PLAYER_POS_RING")]
    RqFilterPlayerPosRing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FilterLogicType {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
}
