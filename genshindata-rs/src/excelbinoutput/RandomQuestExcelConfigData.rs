// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RandomQuestExcelConfigData = Vec<RandomQuestExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomQuestExcelConfigDatum {
    #[serde(rename = "subId")]
    pub sub_id: i64,

    #[serde(rename = "mainId")]
    pub main_id: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "acceptCond")]
    pub accept_cond: Vec<AcceptCond>,

    #[serde(rename = "finishCond")]
    pub finish_cond: Vec<Cond>,

    #[serde(rename = "failCond")]
    pub fail_cond: Vec<Cond>,

    #[serde(rename = "guide")]
    pub guide: Guide,

    #[serde(rename = "guideHint")]
    pub guide_hint: GuideHint,

    #[serde(rename = "failParent")]
    pub fail_parent: Option<bool>,

    #[serde(rename = "failParentShow")]
    pub fail_parent_show: Option<FailParentShow>,

    #[serde(rename = "awardItems")]
    pub award_items: Vec<GuideHint>,

    #[serde(rename = "beginExec")]
    pub begin_exec: Vec<AcceptCond>,

    #[serde(rename = "finishExec")]
    pub finish_exec: Vec<AcceptCond>,

    #[serde(rename = "failExec")]
    pub fail_exec: Vec<AcceptCond>,

    #[serde(rename = "isRewind")]
    pub is_rewind: Option<bool>,

    #[serde(rename = "showType")]
    pub show_type: Option<FailParentShow>,

    #[serde(rename = "finishCondComb")]
    pub finish_cond_comb: Option<FinishCondComb>,

    #[serde(rename = "finishParent")]
    pub finish_parent: Option<bool>,

    #[serde(rename = "exclusiveNpcPriority")]
    pub exclusive_npc_priority: Option<i64>,

    #[serde(rename = "acceptCondComb")]
    pub accept_cond_comb: Option<AcceptCondComb>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptCond {
    #[serde(rename = "param")]
    pub param: Vec<String>,

    #[serde(rename = "type")]
    pub accept_cond_type: Option<AcceptCondType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuideHint {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "type")]
    pub cond_type: Option<FailCondType>,

    #[serde(rename = "param")]
    pub param: Vec<String>,

    #[serde(rename = "param_str")]
    pub param_str: ParamStr,

    #[serde(rename = "count")]
    pub count: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Guide {
    #[serde(rename = "type")]
    pub guide_type: Option<GuideType>,

    #[serde(rename = "param")]
    pub param: Vec<String>,

    #[serde(rename = "guideScene")]
    pub guide_scene: Option<i64>,

    #[serde(rename = "guideStyle")]
    pub guide_style: Option<GuideStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum AcceptCondComb {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum ParamStr {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "30033_fail")]
    The30033_Fail,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FailParentShow {
    #[serde(rename = "QUEST_HIDDEN")]
    QuestHidden,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FinishCondComb {
    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GuideStyle {
    #[serde(rename = "QUEST_GUIDE_STYLE_POINT")]
    QuestGuideStylePoint,

    #[serde(rename = "QUEST_GUIDE_STYLE_TARGET")]
    QuestGuideStyleTarget,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GuideType {
    #[serde(rename = "QUEST_GUIDE_LOCATION")]
    QuestGuideLocation,

    #[serde(rename = "QUEST_GUIDE_NPC")]
    QuestGuideNpc,
}

pub fn load() -> Result<RandomQuestExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RandomQuestExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
