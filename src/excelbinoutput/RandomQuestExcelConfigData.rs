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

pub type RandomQuestExcelConfigData = Vec<RandomQuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RandomQuestExcelConfigDatum {
    #[serde(rename = "_subId")]
    pub sub_id: i64,

    #[serde(rename = "_mainId")]
    pub main_id: i64,

    #[serde(rename = "_order")]
    pub order: i64,

    #[serde(rename = "_titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "_descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "_acceptCond")]
    pub accept_cond: Vec<AcceptCond>,

    #[serde(rename = "_finishCond")]
    pub finish_cond: Vec<Cond>,

    #[serde(rename = "_failCond")]
    pub fail_cond: Vec<Cond>,

    #[serde(rename = "_guide")]
    pub guide: Guide,

    #[serde(rename = "_guideHint")]
    pub guide_hint: GuideHint,

    #[serde(rename = "_failParent")]
    pub fail_parent: Option<bool>,

    #[serde(rename = "_failParentShow")]
    pub fail_parent_show: Option<FailParentShow>,

    #[serde(rename = "_awardItems")]
    pub award_items: Vec<GuideHint>,

    #[serde(rename = "_beginExec")]
    pub begin_exec: Vec<AcceptCond>,

    #[serde(rename = "_finishExec")]
    pub finish_exec: Vec<AcceptCond>,

    #[serde(rename = "_failExec")]
    pub fail_exec: Vec<AcceptCond>,

    #[serde(rename = "_isRewind")]
    pub is_rewind: Option<bool>,

    #[serde(rename = "_showType")]
    pub show_type: Option<FailParentShow>,

    #[serde(rename = "_finishCondComb")]
    pub finish_cond_comb: Option<FinishCondComb>,

    #[serde(rename = "_finishParent")]
    pub finish_parent: Option<bool>,

    #[serde(rename = "_exclusiveNpcPriority")]
    pub exclusive_npc_priority: Option<i64>,

    #[serde(rename = "_acceptCondComb")]
    pub accept_cond_comb: Option<AcceptCondComb>,
}

#[derive(Serialize, Deserialize)]
pub struct AcceptCond {
    #[serde(rename = "_param")]
    pub param: Vec<String>,

    #[serde(rename = "_type")]
    pub accept_cond_type: Option<AcceptCondType>,
}

#[derive(Serialize, Deserialize)]
pub struct GuideHint {}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "_type")]
    pub cond_type: Option<FailCondType>,

    #[serde(rename = "_param")]
    pub param: Vec<String>,

    #[serde(rename = "_param_str")]
    pub param_str: ParamStr,

    #[serde(rename = "_count")]
    pub count: String,
}

#[derive(Serialize, Deserialize)]
pub struct Guide {
    #[serde(rename = "type")]
    pub guide_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: Vec<String>,

    #[serde(rename = "guideScene")]
    pub guide_scene: Option<i64>,

    #[serde(rename = "guideStyle")]
    pub guide_style: Option<GuideStyle>,
}

#[derive(Serialize, Deserialize)]
pub enum AcceptCondType {
    #[serde(rename = "QUEST_COND_ITEM_GIVING_ACTIVED")]
    QuestCondItemGivingActived,

    #[serde(rename = "QUEST_COND_ITEM_GIVING_FINISHED")]
    QuestCondItemGivingFinished,

    #[serde(rename = "QUEST_COND_QUEST_GLOBAL_VAR_EQUAL")]
    QuestCondQuestGlobalVarEqual,

    #[serde(rename = "QUEST_COND_STATE_EQUAL")]
    QuestCondStateEqual,

    #[serde(rename = "QUEST_EXEC_ACTIVE_ITEM_GIVING")]
    QuestExecActiveItemGiving,

    #[serde(rename = "QUEST_EXEC_CREATE_PATTERN_GROUP")]
    QuestExecCreatePatternGroup,

    #[serde(rename = "QUEST_EXEC_DEL_PACK_ITEM")]
    QuestExecDelPackItem,

    #[serde(rename = "QUEST_EXEC_REMOVE_PATTERN_GROUP")]
    QuestExecRemovePatternGroup,

    #[serde(rename = "QUEST_EXEC_SET_QUEST_GLOBAL_VAR")]
    QuestExecSetQuestGlobalVar,
}

#[derive(Serialize, Deserialize)]
pub enum AcceptCondComb {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
}

#[derive(Serialize, Deserialize)]
pub enum FailCondType {
    #[serde(rename = "QUEST_CONTENT_COMPLETE_TALK")]
    QuestContentCompleteTalk,

    #[serde(rename = "QUEST_CONTENT_FINISH_ITEM_GIVING")]
    QuestContentFinishItemGiving,

    #[serde(rename = "QUEST_CONTENT_FINISH_PLOT")]
    QuestContentFinishPlot,

    #[serde(rename = "QUEST_CONTENT_LUA_NOTIFY")]
    QuestContentLuaNotify,

    #[serde(rename = "QUEST_CONTENT_NOT_FINISH_PLOT")]
    QuestContentNotFinishPlot,

    #[serde(rename = "QUEST_CONTENT_PATTERN_GROUP_CLEAR_MONSTER")]
    QuestContentPatternGroupClearMonster,
}

#[derive(Serialize, Deserialize)]
pub enum ParamStr {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "30033_fail")]
    The30033_Fail,
}

#[derive(Serialize, Deserialize)]
pub enum FailParentShow {
    #[serde(rename = "QUEST_HIDDEN")]
    QuestHidden,
}

#[derive(Serialize, Deserialize)]
pub enum FinishCondComb {
    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}

#[derive(Serialize, Deserialize)]
pub enum GuideStyle {
    #[serde(rename = "QUEST_GUIDE_STYLE_POINT")]
    QuestGuideStylePoint,

    #[serde(rename = "QUEST_GUIDE_STYLE_TARGET")]
    QuestGuideStyleTarget,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_GUIDE_LOCATION")]
    QuestGuideLocation,

    #[serde(rename = "QUEST_GUIDE_NPC")]
    QuestGuideNpc,
}
