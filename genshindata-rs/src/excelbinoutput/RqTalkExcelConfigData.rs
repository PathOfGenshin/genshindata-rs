// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type RqTalkExcelConfigData = Vec<RqTalkExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RqTalkExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "beginWay")]
    pub begin_way: Option<BeginWay>,

    #[serde(rename = "beginCond")]
    pub begin_cond: Vec<BeginCond>,

    #[serde(rename = "priority")]
    pub priority: Option<i64>,

    #[serde(rename = "nextTalks")]
    pub next_talks: Vec<i64>,

    #[serde(rename = "nextRandomTalks")]
    pub next_random_talks: Vec<i64>,

    #[serde(rename = "initDialog")]
    pub init_dialog: i64,

    #[serde(rename = "npcId")]
    pub npc_id: Vec<i64>,

    #[serde(rename = "performCfg")]
    pub perform_cfg: String,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "beginCondComb")]
    pub begin_cond_comb: Option<BeginCondComb>,

    #[serde(rename = "heroTalk")]
    pub hero_talk: Option<HeroTalk>,

    #[serde(rename = "showRandomTalkCount")]
    pub show_random_talk_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginCond {
    #[serde(rename = "type")]
    pub begin_cond_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_COND_ITEM_GIVING_ACTIVED")]
    QuestCondItemGivingActived,

    #[serde(rename = "QUEST_COND_ITEM_GIVING_FINISHED")]
    QuestCondItemGivingFinished,

    #[serde(rename = "QUEST_COND_ITEM_NUM_LESS_THAN")]
    QuestCondItemNumLessThan,

    #[serde(rename = "QUEST_COND_PACK_HAVE_ITEM")]
    QuestCondPackHaveItem,

    #[serde(rename = "QUEST_COND_QUEST_GLOBAL_VAR_EQUAL")]
    QuestCondQuestGlobalVarEqual,

    #[serde(rename = "QUEST_COND_STATE_EQUAL")]
    QuestCondStateEqual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BeginCondComb {
    #[serde(rename = "LOGIC_A_AND_ETCOR")]
    LogicAAndEtcor,

    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BeginWay {
    #[serde(rename = "TALK_BEGIN_MANUAL")]
    TalkBeginManual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HeroTalk {
    #[serde(rename = "TALK_HERO_MAIN")]
    TalkHeroMain,
}

pub fn load() -> Result<RqTalkExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RqTalkExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
