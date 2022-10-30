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

pub type TalkExcelConfigData = Vec<TalkExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TalkExcelConfigDatum {
    #[serde(rename = "_id")]
    pub id: i64,

    #[serde(rename = "_beginWay")]
    pub begin_way: Option<BeginWay>,

    #[serde(rename = "_activeMode")]
    pub active_mode: Option<ActiveMode>,

    #[serde(rename = "_beginCond")]
    pub begin_cond: Vec<BeginCond>,

    #[serde(rename = "_priority")]
    pub priority: Option<i64>,

    #[serde(rename = "_nextTalks")]
    pub next_talks: Vec<i64>,

    #[serde(rename = "_initDialog")]
    pub init_dialog: Option<i64>,

    #[serde(rename = "_nextRandomTalks")]
    pub next_random_talks: Vec<i64>,

    #[serde(rename = "_npcId")]
    pub npc_id: Vec<i64>,

    #[serde(rename = "_participantId")]
    pub participant_id: Vec<i64>,

    #[serde(rename = "_performCfg")]
    pub perform_cfg: String,

    #[serde(rename = "_extraLoadMarkId")]
    pub extra_load_mark_id: Vec<i64>,

    #[serde(rename = "_prePerformCfg")]
    pub pre_perform_cfg: PrePerformCfg,

    #[serde(rename = "_talkMarkHideList")]
    pub talk_mark_hide_list: Vec<i64>,

    #[serde(rename = "_crowdLOD0List")]
    pub crowd_lod0_list: Vec<i64>,

    #[serde(rename = "_finishExec")]
    pub finish_exec: Vec<BeginCond>,

    #[serde(rename = "_beginCondComb")]
    pub begin_cond_comb: Option<BeginCondComb>,

    #[serde(rename = "_questId")]
    pub quest_id: Option<i64>,

    #[serde(rename = "_heroTalk")]
    pub hero_talk: Option<HeroTalk>,

    #[serde(rename = "_talkMarkType")]
    pub talk_mark_type: Option<TalkMarkType>,

    #[serde(rename = "_dontBlockDaily")]
    pub dont_block_daily: Option<bool>,

    #[serde(rename = "_lowPriority")]
    pub low_priority: Option<bool>,

    #[serde(rename = "_lockGameTime")]
    pub lock_game_time: Option<bool>,

    #[serde(rename = "_showRandomTalkCount")]
    pub show_random_talk_count: Option<i64>,

    #[serde(rename = "_stayFreeStyle")]
    pub stay_free_style: Option<bool>,

    #[serde(rename = "_loadType")]
    pub load_type: Option<LoadType>,

    #[serde(rename = "_checkActionAfter")]
    pub check_action_after: Option<bool>,

    #[serde(rename = "_enableCameraDisplacement")]
    pub enable_camera_displacement: Option<bool>,

    #[serde(rename = "_questIdleTalk")]
    pub quest_idle_talk: Option<bool>,

    #[serde(rename = "_decoratorID")]
    pub decorator_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct BeginCond {
    #[serde(rename = "_param")]
    pub param: Vec<String>,

    #[serde(rename = "_type")]
    pub begin_cond_type: Option<Type>,
}

#[derive(Serialize, Deserialize)]
pub enum ActiveMode {
    #[serde(rename = "PLAY_MODE_GUEST")]
    PlayModeGuest,

    #[serde(rename = "PLAY_MODE_HOST")]
    PlayModeHost,

    #[serde(rename = "PLAY_MODE_SINGLE")]
    PlayModeSingle,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_COND_ACTIVITY_CLIENT_COND")]
    QuestCondActivityClientCond,

    #[serde(rename = "QUEST_COND_ACTIVITY_OPEN")]
    QuestCondActivityOpen,

    #[serde(rename = "QUEST_COND_AVATAR_CAN_CHANGE_ELEMENT")]
    QuestCondAvatarCanChangeElement,

    #[serde(rename = "QUEST_COND_AVATAR_ELEMENT_NOT_EQUAL")]
    QuestCondAvatarElementNotEqual,

    #[serde(rename = "QUEST_COND_AVATAR_FETTER_EQ")]
    QuestCondAvatarFetterEq,

    #[serde(rename = "QUEST_COND_AVATAR_FETTER_GT")]
    QuestCondAvatarFetterGt,

    #[serde(rename = "QUEST_COND_AVATAR_FETTER_LT")]
    QuestCondAvatarFetterLt,

    #[serde(rename = "QUEST_COND_CITY_LEVEL_EQUAL_GREATER")]
    QuestCondCityLevelEqualGreater,

    #[serde(rename = "QUEST_COND_CITY_REPUTATION_LEVEL")]
    QuestCondCityReputationLevel,

    #[serde(rename = "QUEST_COND_DAILY_TASK_IN_PROGRESS")]
    QuestCondDailyTaskInProgress,

    #[serde(rename = "QUEST_COND_DAILY_TASK_OPEN")]
    QuestCondDailyTaskOpen,

    #[serde(rename = "QUEST_COND_DAILY_TASK_REWARD_CAN_GET")]
    QuestCondDailyTaskRewardCanGet,

    #[serde(rename = "QUEST_COND_DAILY_TASK_REWARD_RECEIVED")]
    QuestCondDailyTaskRewardReceived,

    #[serde(rename = "QUEST_COND_DAILY_TASK_VAR_EQ")]
    QuestCondDailyTaskVarEq,

    #[serde(rename = "QUEST_COND_DAILY_TASK_VAR_GT")]
    QuestCondDailyTaskVarGt,

    #[serde(rename = "QUEST_COND_DAILY_TASK_VAR_LT")]
    QuestCondDailyTaskVarLt,

    #[serde(rename = "QUEST_COND_EXPLORATION_REWARD_CAN_GET")]
    QuestCondExplorationRewardCanGet,

    #[serde(rename = "QUEST_COND_FORGE_HAVE_FINISH")]
    QuestCondForgeHaveFinish,

    #[serde(rename = "QUEST_COND_GADGET_TALK_STATE_EQUAL")]
    QuestCondGadgetTalkStateEqual,

    #[serde(rename = "QUEST_COND_HOMEWORLD_NPC_EVENT")]
    QuestCondHomeworldNpcEvent,

    #[serde(rename = "QUEST_COND_HOMEWORLD_NPC_NEW_TALK")]
    QuestCondHomeworldNpcNewTalk,

    #[serde(rename = "QUEST_COND_IS_CUR_BLOSSOM_TALK")]
    QuestCondIsCurBlossomTalk,

    #[serde(rename = "QUEST_COND_IS_DAYTIME")]
    QuestCondIsDaytime,

    #[serde(rename = "QUEST_COND_IS_WORLD_OWNER")]
    QuestCondIsWorldOwner,

    #[serde(rename = "QUEST_COND_ITEM_GIVING_ACTIVED")]
    QuestCondItemGivingActived,

    #[serde(rename = "QUEST_COND_ITEM_GIVING_FINISHED")]
    QuestCondItemGivingFinished,

    #[serde(rename = "QUEST_COND_ITEM_NUM_LESS_THAN")]
    QuestCondItemNumLessThan,

    #[serde(rename = "QUEST_COND_LUA_NOTIFY")]
    QuestCondLuaNotify,

    #[serde(rename = "QUEST_COND_LUNARITE_COLLECT_FINISH")]
    QuestCondLunariteCollectFinish,

    #[serde(rename = "QUEST_COND_LUNARITE_HAS_REGION_HINT_COUNT")]
    QuestCondLunariteHasRegionHintCount,

    #[serde(rename = "QUEST_COND_LUNARITE_MARK_ALL_FINISH")]
    QuestCondLunariteMarkAllFinish,

    #[serde(rename = "QUEST_COND_LUNARITE_REGION_UNLOCKED")]
    QuestCondLunariteRegionUnlocked,

    #[serde(rename = "QUEST_COND_NEW_HOMEWORLD_LEVEL_REWARD")]
    QuestCondNewHomeworldLevelReward,

    #[serde(rename = "QUEST_COND_NEW_HOMEWORLD_MAKE_FINISH")]
    QuestCondNewHomeworldMakeFinish,

    #[serde(rename = "QUEST_COND_NEW_HOMEWORLD_MOUDLE_UNLOCK")]
    QuestCondNewHomeworldMoudleUnlock,

    #[serde(rename = "QUEST_COND_NEW_HOMEWORLD_SHOP_ITEM")]
    QuestCondNewHomeworldShopItem,

    #[serde(rename = "QUEST_COND_NEW_HOMEWORLD_WOOD_EXCHANGE_UNLOCK")]
    QuestCondNewHomeworldWoodExchangeUnlock,

    #[serde(rename = "QUEST_COND_NOT_HAVE_BLOSSOM_TALK")]
    QuestCondNotHaveBlossomTalk,

    #[serde(rename = "QUEST_COND_OPEN_STATE_EQUAL")]
    QuestCondOpenStateEqual,

    #[serde(rename = "QUEST_COND_PACK_HAVE_ITEM")]
    QuestCondPackHaveItem,

    #[serde(rename = "QUEST_COND_PLAYER_CHOOSE_MALE")]
    QuestCondPlayerChooseMale,

    #[serde(rename = "QUEST_COND_PLAYER_LEVEL_REWARD_CAN_GET")]
    QuestCondPlayerLevelRewardCanGet,

    #[serde(rename = "QUEST_COND_QUEST_GLOBAL_VAR_EQUAL")]
    QuestCondQuestGlobalVarEqual,

    #[serde(rename = "QUEST_COND_QUEST_GLOBAL_VAR_GREATER")]
    QuestCondQuestGlobalVarGreater,

    #[serde(rename = "QUEST_COND_QUEST_GLOBAL_VAR_LESS")]
    QuestCondQuestGlobalVarLess,

    #[serde(rename = "QUEST_COND_QUEST_NOT_RECEIVE")]
    QuestCondQuestNotReceive,

    #[serde(rename = "QUEST_COND_QUEST_SERVER_COND_VALID")]
    QuestCondQuestServerCondValid,

    #[serde(rename = "QUEST_COND_QUEST_VAR_EQUAL")]
    QuestCondQuestVarEqual,

    #[serde(rename = "QUEST_COND_QUEST_VAR_GREATER")]
    QuestCondQuestVarGreater,

    #[serde(rename = "QUEST_COND_QUEST_VAR_LESS")]
    QuestCondQuestVarLess,

    #[serde(rename = "QUEST_COND_SCENE_AREA_UNLOCKED")]
    QuestCondSceneAreaUnlocked,

    #[serde(rename = "QUEST_COND_SCENE_LEVEL_TAG_EQ")]
    QuestCondSceneLevelTagEq,

    #[serde(rename = "QUEST_COND_SCENE_POINT_UNLOCK")]
    QuestCondScenePointUnlock,

    #[serde(rename = "QUEST_COND_STATE_EQUAL")]
    QuestCondStateEqual,

    #[serde(rename = "QUEST_COND_STATE_NOT_EQUAL")]
    QuestCondStateNotEqual,

    #[serde(rename = "TALK_EXEC_DEC_QUEST_GLOBAL_VAR")]
    TalkExecDecQuestGlobalVar,

    #[serde(rename = "TALK_EXEC_DEC_QUEST_VAR")]
    TalkExecDecQuestVar,

    #[serde(rename = "TALK_EXEC_INC_DAILY_TASK_VAR")]
    TalkExecIncDailyTaskVar,

    #[serde(rename = "TALK_EXEC_INC_QUEST_GLOBAL_VAR")]
    TalkExecIncQuestGlobalVar,

    #[serde(rename = "TALK_EXEC_INC_QUEST_VAR")]
    TalkExecIncQuestVar,

    #[serde(rename = "TALK_EXEC_NOTIFY_GROUP_LUA")]
    TalkExecNotifyGroupLua,

    #[serde(rename = "TALK_EXEC_SAVE_TALK_ID")]
    TalkExecSaveTalkId,

    #[serde(rename = "TALK_EXEC_SET_DAILY_TASK_VAR")]
    TalkExecSetDailyTaskVar,

    #[serde(rename = "TALK_EXEC_SET_GADGET_STATE")]
    TalkExecSetGadgetState,

    #[serde(rename = "TALK_EXEC_SET_GAME_TIME")]
    TalkExecSetGameTime,

    #[serde(rename = "TALK_EXEC_SET_QUEST_GLOBAL_VAR")]
    TalkExecSetQuestGlobalVar,

    #[serde(rename = "TALK_EXEC_SET_QUEST_VAR")]
    TalkExecSetQuestVar,

    #[serde(rename = "TALK_EXEC_TRANS_SCENE_DUMMY_POINT")]
    TalkExecTransSceneDummyPoint,
}

#[derive(Serialize, Deserialize)]
pub enum BeginCondComb {
    #[serde(rename = "LOGIC_A_AND_B_AND_ETCOR")]
    LogicAAndBAndEtcor,

    #[serde(rename = "LOGIC_A_AND_B_OR_ETCAND")]
    LogicAAndBOrEtcand,

    #[serde(rename = "LOGIC_A_AND_ETCOR")]
    LogicAAndEtcor,

    #[serde(rename = "LOGIC_A_OR_B_OR_ETCAND")]
    LogicAOrBOrEtcand,

    #[serde(rename = "LOGIC_A_OR_ETCAND")]
    LogicAOrEtcand,

    #[serde(rename = "LOGIC_AND")]
    LogicAnd,

    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}

#[derive(Serialize, Deserialize)]
pub enum BeginWay {
    #[serde(rename = "TALK_BEGIN_AUTO")]
    TalkBeginAuto,

    #[serde(rename = "TALK_BEGIN_MANUAL")]
    TalkBeginManual,
}

#[derive(Serialize, Deserialize)]
pub enum HeroTalk {
    #[serde(rename = "TALK_HERO_MAIN")]
    TalkHeroMain,
}

#[derive(Serialize, Deserialize)]
pub enum LoadType {
    #[serde(rename = "TALK_ACTIVITY")]
    TalkActivity,

    #[serde(rename = "TALK_BLOSSOM")]
    TalkBlossom,

    #[serde(rename = "TALK_GADGET")]
    TalkGadget,
}

#[derive(Serialize, Deserialize)]
pub enum PrePerformCfg {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "QuestDialogue/COOP/CoopBarbara/Q21024")]
    QuestDialogueCoopCoopBarbaraQ21024,

    #[serde(rename = "QuestDialogue/COOP/CoopBarbara/test1")]
    QuestDialogueCoopCoopBarbaraTest1,

    #[serde(rename = "QuestDialogue/COOP/CoopBarbara/test2")]
    QuestDialogueCoopCoopBarbaraTest2,

    #[serde(rename = "QuestDialogue/COOP/CoopBarbara/test3")]
    QuestDialogueCoopCoopBarbaraTest3,

    #[serde(rename = "QuestDialogue/WQ/Inazuma_22016/Q22016_Talk")]
    QuestDialogueWqInazuma22016Q22016Talk,

    #[serde(rename = "QuestDialogue/WQ/Liyue_70815/Q70815_Talk")]
    QuestDialogueWqLiyue70815Q70815Talk,

    #[serde(rename = "QuestDialogue/WQ/Liyue_70822/Q70822_Talk")]
    QuestDialogueWqLiyue70822Q70822Talk,

    #[serde(rename = "QuestDialogue/WQ/Liyue_70823/Q70823_Talk")]
    QuestDialogueWqLiyue70823Q70823Talk,

    #[serde(rename = "QuestDialogue/WQ/Liyue_71020/Q71020_Talk")]
    QuestDialogueWqLiyue71020Q71020Talk,

    #[serde(rename = "QuestDialogue/WQ/Liyue_71800/Q71800_Talk")]
    QuestDialogueWqLiyue71800Q71800Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_20601/Q20601_Talk")]
    QuestDialogueWqMengde20601Q20601Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_20602/Q20602_Talk")]
    QuestDialogueWqMengde20602Q20602Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_20603/Q20603_Talk")]
    QuestDialogueWqMengde20603Q20603Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_20740/Q20740_Talk")]
    QuestDialogueWqMengde20740Q20740Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_20741/Q20741_Talk")]
    QuestDialogueWqMengde20741Q20741Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_20743/Q20743_Talk")]
    QuestDialogueWqMengde20743Q20743Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_70801/Q70801_Talk")]
    QuestDialogueWqMengde70801Q70801Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_70810/Q70810_Talk")]
    QuestDialogueWqMengde70810Q70810Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_70812/Q70812_Talk")]
    QuestDialogueWqMengde70812Q70812Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_70813/Q70813_Talk")]
    QuestDialogueWqMengde70813Q70813Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_70816/Q70816_Talk")]
    QuestDialogueWqMengde70816Q70816Talk,

    #[serde(rename = "QuestDialogue/WQ/Mengde_70824/Q70824_Talk")]
    QuestDialogueWqMengde70824Q70824Talk,
}

#[derive(Serialize, Deserialize)]
pub enum TalkMarkType {
    #[serde(rename = "TALK_MARK_COMMON")]
    TalkMarkCommon,

    #[serde(rename = "TALK_MARK_HIDE")]
    TalkMarkHide,
}
