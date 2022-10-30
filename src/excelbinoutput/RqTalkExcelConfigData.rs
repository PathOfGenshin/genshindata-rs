// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type RqTalkExcelConfigData = Vec<RqTalkExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RqTalkExcelConfigDatum {
    #[serde(rename = "_id")]
    pub id: i64,

    #[serde(rename = "_beginWay")]
    pub begin_way: Option<BeginWay>,

    #[serde(rename = "_beginCond")]
    pub begin_cond: Vec<BeginCond>,

    #[serde(rename = "_priority")]
    pub priority: Option<i64>,

    #[serde(rename = "_nextTalks")]
    pub next_talks: Vec<i64>,

    #[serde(rename = "_nextRandomTalks")]
    pub next_random_talks: Vec<i64>,

    #[serde(rename = "_initDialog")]
    pub init_dialog: i64,

    #[serde(rename = "_npcId")]
    pub npc_id: Vec<i64>,

    #[serde(rename = "_performCfg")]
    pub perform_cfg: String,

    #[serde(rename = "_questId")]
    pub quest_id: i64,

    #[serde(rename = "_beginCondComb")]
    pub begin_cond_comb: Option<BeginCondComb>,

    #[serde(rename = "_heroTalk")]
    pub hero_talk: Option<HeroTalk>,

    #[serde(rename = "_showRandomTalkCount")]
    pub show_random_talk_count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct BeginCond {
    #[serde(rename = "_type")]
    pub begin_cond_type: Option<Type>,

    #[serde(rename = "_param")]
    pub param: Vec<String>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum BeginCondComb {
    #[serde(rename = "LOGIC_A_AND_ETCOR")]
    LogicAAndEtcor,

    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
}

#[derive(Serialize, Deserialize)]
pub enum BeginWay {
    #[serde(rename = "TALK_BEGIN_MANUAL")]
    TalkBeginManual,
}

#[derive(Serialize, Deserialize)]
pub enum HeroTalk {
    #[serde(rename = "TALK_HERO_MAIN")]
    TalkHeroMain,
}
