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

pub type DialogExcelConfigData = Vec<DialogExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DialogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nextDialogs")]
    pub next_dialogs: Vec<i64>,

    #[serde(rename = "talkRole")]
    pub talk_role: TalkRole,

    #[serde(rename = "talkContentTextMapHash")]
    pub talk_content_text_map_hash: i64,

    #[serde(rename = "talkTitleTextMapHash")]
    pub talk_title_text_map_hash: i64,

    #[serde(rename = "talkRoleNameTextMapHash")]
    pub talk_role_name_text_map_hash: i64,

    #[serde(rename = "talkAssetPath")]
    pub talk_asset_path: String,

    #[serde(rename = "talkAssetPathAlter")]
    pub talk_asset_path_alter: String,

    #[serde(rename = "talkAudioName")]
    pub talk_audio_name: String,

    #[serde(rename = "actionBefore")]
    pub action_before: ActionBefore,

    #[serde(rename = "actionWhile")]
    pub action_while: ActionWhile,

    #[serde(rename = "actionAfter")]
    pub action_after: ActionAfter,

    #[serde(rename = "optionIcon")]
    pub option_icon: OptionIcon,

    #[serde(rename = "talkShowType")]
    pub talk_show_type: Option<TalkShowType>,

    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,

    #[serde(rename = "showDuration")]
    pub show_duration: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct TalkRole {
    #[serde(rename = "_type")]
    pub talk_role_type: Option<Type>,

    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub enum ActionAfter {
    #[serde(rename = "DialogAction/D220420109_After")]
    DialogActionD220420109After,

    #[serde(rename = "DialogAction/D400650703_After")]
    DialogActionD400650703After,

    #[serde(rename = "DialogAction/D400650903_After")]
    DialogActionD400650903After,

    #[serde(rename = "DialogAction/D400770202_After")]
    DialogActionD400770202After,

    #[serde(rename = "DialogAction/D710550827_After")]
    DialogActionD710550827After,

    #[serde(rename = "DialogAction/D721220112_After")]
    DialogActionD721220112After,

    #[serde(rename = "DialogAction/D721670307_After")]
    DialogActionD721670307After,

    #[serde(rename = "DialogAction/D721670309_After")]
    DialogActionD721670309After,

    #[serde(rename = "DialogAction/D721670403_After")]
    DialogActionD721670403After,

    #[serde(rename = "DialogAction/D721671610_After")]
    DialogActionD721671610After,

    #[serde(rename = "DialogAction/D721671612_After")]
    DialogActionD721671612After,

    #[serde(rename = "DialogAction/D721672003_After")]
    DialogActionD721672003After,

    #[serde(rename = "DialogAction/D722430119_After")]
    DialogActionD722430119After,

    #[serde(rename = "DialogAction/D722430122_After")]
    DialogActionD722430122After,

    #[serde(rename = "DialogAction/D722430209_After")]
    DialogActionD722430209After,

    #[serde(rename = "DialogAction/D722710118_After")]
    DialogActionD722710118After,

    #[serde(rename = "DialogAction/D722710121_After")]
    DialogActionD722710121After,

    #[serde(rename = "DialogAction/D722710206_After")]
    DialogActionD722710206After,

    #[serde(rename = "DialogAction/D722720116_After")]
    DialogActionD722720116After,

    #[serde(rename = "DialogAction/D722720119_After")]
    DialogActionD722720119After,

    #[serde(rename = "DialogAction/D722720205_After")]
    DialogActionD722720205After,

    #[serde(rename = "DialogAction/D790230810_After")]
    DialogActionD790230810After,

    #[serde(rename = "")]
    Empty,

    #[serde(rename = "HomeWorld/huling_finish")]
    HomeWorldHulingFinish,

    #[serde(rename = "QuestDialogue/IQ/Inazuma_22014/Q2201402")]
    QuestDialogueIqInazuma22014Q2201402,

    #[serde(rename = "QuestDialogue/IQ/Inazuma_22014/Q2201403")]
    QuestDialogueIqInazuma22014Q2201403,

    #[serde(rename = "QuestDialogue/IQ/Inazuma_22022/Q2202201")]
    QuestDialogueIqInazuma22022Q2202201,

    #[serde(rename = "QuestDialogue/IQ/Inazuma_22022/Q2202202")]
    QuestDialogueIqInazuma22022Q2202202,

    #[serde(rename = "QuestDialogue/IQ/Inazuma_22030/Q2203005")]
    QuestDialogueIqInazuma22030Q2203005,

    #[serde(rename = "QuestDialogue/IQ/Inazuma_22036/Q2203601")]
    QuestDialogueIqInazuma22036Q2203601,

    #[serde(rename = "QuestDialogue/IQ/Inazuma_22036/Q2203602")]
    QuestDialogueIqInazuma22036Q2203602,

    #[serde(rename = "QuestDialogue/IQ/Inazuma_22036/Q2203603")]
    QuestDialogueIqInazuma22036Q2203603,

    #[serde(rename = "Shop/shop_open_1002")]
    ShopShopOpen1002,

    #[serde(rename = "Shop/shop_open_1003")]
    ShopShopOpen1003,

    #[serde(rename = "Shop/shop_open_1004")]
    ShopShopOpen1004,

    #[serde(rename = "Shop/shop_open_1005")]
    ShopShopOpen1005,

    #[serde(rename = "Shop/shop_open_1006")]
    ShopShopOpen1006,

    #[serde(rename = "Shop/shop_open_1008")]
    ShopShopOpen1008,

    #[serde(rename = "Shop/shop_open_1009")]
    ShopShopOpen1009,

    #[serde(rename = "Shop/shop_open_1010")]
    ShopShopOpen1010,

    #[serde(rename = "Shop/shop_open_1011")]
    ShopShopOpen1011,

    #[serde(rename = "Shop/shop_open_1012")]
    ShopShopOpen1012,

    #[serde(rename = "Shop/shop_open_1013")]
    ShopShopOpen1013,

    #[serde(rename = "Shop/shop_open_1014")]
    ShopShopOpen1014,

    #[serde(rename = "Shop/shop_open_1015")]
    ShopShopOpen1015,

    #[serde(rename = "Shop/shop_open_1016")]
    ShopShopOpen1016,

    #[serde(rename = "Shop/shop_open_1017")]
    ShopShopOpen1017,

    #[serde(rename = "Shop/shop_open_1018")]
    ShopShopOpen1018,

    #[serde(rename = "Shop/shop_open_1019")]
    ShopShopOpen1019,

    #[serde(rename = "Shop/shop_open_1020")]
    ShopShopOpen1020,

    #[serde(rename = "Shop/shop_open_1021")]
    ShopShopOpen1021,

    #[serde(rename = "Shop/shop_open_1022")]
    ShopShopOpen1022,

    #[serde(rename = "Shop/shop_open_1023")]
    ShopShopOpen1023,

    #[serde(rename = "Shop/shop_open_1024")]
    ShopShopOpen1024,

    #[serde(rename = "Shop/shop_open_1025")]
    ShopShopOpen1025,

    #[serde(rename = "Shop/shop_open_1026")]
    ShopShopOpen1026,

    #[serde(rename = "Shop/shop_open_1027")]
    ShopShopOpen1027,

    #[serde(rename = "Shop/shop_open_1028")]
    ShopShopOpen1028,

    #[serde(rename = "Shop/shop_open_1029")]
    ShopShopOpen1029,

    #[serde(rename = "Shop/shop_open_1030")]
    ShopShopOpen1030,

    #[serde(rename = "Shop/shop_open_1031")]
    ShopShopOpen1031,

    #[serde(rename = "Shop/shop_open_1032")]
    ShopShopOpen1032,

    #[serde(rename = "Shop/shop_open_1033")]
    ShopShopOpen1033,

    #[serde(rename = "Shop/shop_open_1034")]
    ShopShopOpen1034,

    #[serde(rename = "Shop/shop_open_1035")]
    ShopShopOpen1035,

    #[serde(rename = "Shop/shop_open_1036")]
    ShopShopOpen1036,

    #[serde(rename = "Shop/shop_open_1038")]
    ShopShopOpen1038,

    #[serde(rename = "Shop/shop_open_1039")]
    ShopShopOpen1039,

    #[serde(rename = "Shop/shop_open_1041")]
    ShopShopOpen1041,

    #[serde(rename = "Shop/shop_open_1042")]
    ShopShopOpen1042,

    #[serde(rename = "Shop/shop_open_1045")]
    ShopShopOpen1045,

    #[serde(rename = "Shop/shop_open_1046")]
    ShopShopOpen1046,

    #[serde(rename = "Shop/shop_open_1047")]
    ShopShopOpen1047,

    #[serde(rename = "Shop/shop_open_1050")]
    ShopShopOpen1050,

    #[serde(rename = "Shop/shop_open_1051")]
    ShopShopOpen1051,

    #[serde(rename = "Shop/shop_open_1053")]
    ShopShopOpen1053,

    #[serde(rename = "Shop/shop_open_1054")]
    ShopShopOpen1054,

    #[serde(rename = "Shop/shop_open_1055")]
    ShopShopOpen1055,

    #[serde(rename = "Shop/shop_open_1056")]
    ShopShopOpen1056,

    #[serde(rename = "Shop/shop_open_1057")]
    ShopShopOpen1057,

    #[serde(rename = "Shop/shop_open_1058")]
    ShopShopOpen1058,

    #[serde(rename = "Shop/shop_open_1059")]
    ShopShopOpen1059,

    #[serde(rename = "Shop/shop_open_1060")]
    ShopShopOpen1060,

    #[serde(rename = "Shop/shop_open_1061")]
    ShopShopOpen1061,

    #[serde(rename = "Shop/shop_open_1062")]
    ShopShopOpen1062,

    #[serde(rename = "Shop/shop_open_1063")]
    ShopShopOpen1063,

    #[serde(rename = "Shop/shop_open_1064")]
    ShopShopOpen1064,

    #[serde(rename = "Shop/shop_open_1065")]
    ShopShopOpen1065,

    #[serde(rename = "Shop/shop_open_1066")]
    ShopShopOpen1066,

    #[serde(rename = "Shop/shop_open_1067")]
    ShopShopOpen1067,

    #[serde(rename = "Shop/shop_open_1068")]
    ShopShopOpen1068,

    #[serde(rename = "Shop/shop_open_1069")]
    ShopShopOpen1069,

    #[serde(rename = "Shop/shop_open_1070")]
    ShopShopOpen1070,

    #[serde(rename = "Shop/shop_open_1071")]
    ShopShopOpen1071,

    #[serde(rename = "Shop/shop_open_1072")]
    ShopShopOpen1072,

    #[serde(rename = "Shop/shop_open_1073")]
    ShopShopOpen1073,

    #[serde(rename = "Shop/shop_open_1074")]
    ShopShopOpen1074,

    #[serde(rename = "Shop/shop_open_1075")]
    ShopShopOpen1075,

    #[serde(rename = "Shop/shop_open_1076")]
    ShopShopOpen1076,

    #[serde(rename = "Shop/shop_open_1077")]
    ShopShopOpen1077,

    #[serde(rename = "Shop/shop_open_1078")]
    ShopShopOpen1078,

    #[serde(rename = "Shop/shop_open_1079")]
    ShopShopOpen1079,

    #[serde(rename = "Shop/shop_open_1080")]
    ShopShopOpen1080,

    #[serde(rename = "Shop/shop_open_1081")]
    ShopShopOpen1081,

    #[serde(rename = "Shop/shop_open_1082")]
    ShopShopOpen1082,

    #[serde(rename = "Shop/shop_open_1083")]
    ShopShopOpen1083,

    #[serde(rename = "Shop/shop_open_1084")]
    ShopShopOpen1084,

    #[serde(rename = "Shop/shop_open_1085")]
    ShopShopOpen1085,

    #[serde(rename = "Shop/shop_open_1086")]
    ShopShopOpen1086,

    #[serde(rename = "Shop/shop_open_1087")]
    ShopShopOpen1087,

    #[serde(rename = "Shop/shop_open_1088")]
    ShopShopOpen1088,

    #[serde(rename = "Shop/shop_open_31001")]
    ShopShopOpen31001,

    #[serde(rename = "SimpleTalk/Downcast")]
    SimpleTalkDowncast,

    #[serde(rename = "SimpleTalk/Greet")]
    SimpleTalkGreet,

    #[serde(rename = "SimpleTalk/Idle_End")]
    SimpleTalkIdleEnd,

    #[serde(rename = "SimpleTalk/LerpOutNpcNoSteer_End")]
    SimpleTalkLerpOutNpcNoSteerEnd,

    #[serde(rename = "SimpleTalk/Open_giving_page_End")]
    SimpleTalkOpenGivingPageEnd,

    #[serde(rename = "SimpleTalk/RandomTalk")]
    SimpleTalkRandomTalk,

    #[serde(rename = "SimpleTalk/Standby")]
    SimpleTalkStandby,

    #[serde(rename = "SimpleTalk/SteerOnly")]
    SimpleTalkSteerOnly,

    #[serde(rename = "TEST/BlossomTalkEnd")]
    TestBlossomTalkEnd,

    #[serde(rename = "TEST/D200490113")]
    TestD200490113,

    #[serde(rename = "TEST/D205030507")]
    TestD205030507,

    #[serde(rename = "TEST/D205040115")]
    TestD205040115,

    #[serde(rename = "TEST/D205050128")]
    TestD205050128,

    #[serde(rename = "TEST/D205060111")]
    TestD205060111,

    #[serde(rename = "TEST/D410010902")]
    TestD410010902,

    #[serde(rename = "TEST/D410011002")]
    TestD410011002,

    #[serde(rename = "TEST/D706500115")]
    TestD706500115,

    #[serde(rename = "TEST/GODDESS_CHROD")]
    TestGoddessChrod,

    #[serde(rename = "TEST/LerpInNpcSteer")]
    TestLerpInNpcSteer,

    #[serde(rename = "TEST/LerpInQueen")]
    TestLerpInQueen,

    #[serde(rename = "TEST/LerpInQueen1201_01")]
    TestLerpInQueen120101,

    #[serde(rename = "TEST/LerpOutNpc_10100")]
    TestLerpOutNpc10100,

    #[serde(rename = "TEST/LerpOutNpcNoSteer")]
    TestLerpOutNpcNoSteer,

    #[serde(rename = "TEST/LerpOutNpcNoSteer_1607")]
    TestLerpOutNpcNoSteer1607,

    #[serde(rename = "TEST/LerpOutNpcNoSteerAndGadgetTouch")]
    TestLerpOutNpcNoSteerAndGadgetTouch,

    #[serde(rename = "TEST/LerpOut_TryAvatar")]
    TestLerpOutTryAvatar,

    #[serde(rename = "TEST/LerpOut_TryAvatar308")]
    TestLerpOutTryAvatar308,

    #[serde(rename = "TEST/Lua_10100_4")]
    TestLua10100_4,

    #[serde(rename = "TEST/Lua_10200_1")]
    TestLua10200_1,

    #[serde(rename = "TEST/Lua_10200_2")]
    TestLua10200_2,

    #[serde(rename = "TEST/Lua_10501_5")]
    TestLua10501_5,

    #[serde(rename = "TEST/Lua_12007_2")]
    TestLua12007_2,

    #[serde(rename = "TEST/Lua_20047_1")]
    TestLua20047_1,

    #[serde(rename = "TEST/Lua_20059_0")]
    TestLua20059_0,

    #[serde(rename = "TEST/Lua_20059_1")]
    TestLua20059_1,

    #[serde(rename = "TEST/Lua_20059_100")]
    TestLua20059_100,

    #[serde(rename = "TEST/Lua_20059_101")]
    TestLua20059_101,

    #[serde(rename = "TEST/Lua_20059_102")]
    TestLua20059_102,

    #[serde(rename = "TEST/Lua_20059_2")]
    TestLua20059_2,

    #[serde(rename = "TEST/Lua_20059_3")]
    TestLua20059_3,

    #[serde(rename = "TEST/Lua_20061_1")]
    TestLua20061_1,

    #[serde(rename = "TEST/Lua_20068_1")]
    TestLua20068_1,

    #[serde(rename = "TEST/Lua_20101_11Fight")]
    TestLua20101_11Fight,

    #[serde(rename = "TEST/Lua_20101_1Fight")]
    TestLua20101_1Fight,

    #[serde(rename = "TEST/Lua_20101_21Fight")]
    TestLua20101_21Fight,

    #[serde(rename = "TEST/Lua_20519_1")]
    TestLua20519_1,

    #[serde(rename = "TEST/Lua_21004_1")]
    TestLua21004_1,

    #[serde(rename = "TEST/Lua_21021_1")]
    TestLua21021_1,

    #[serde(rename = "TEST/Lua_22006_1")]
    TestLua22006_1,

    #[serde(rename = "TEST/Lua_22006_2")]
    TestLua22006_2,

    #[serde(rename = "TEST/Lua_22010_1")]
    TestLua22010_1,

    #[serde(rename = "TEST/Lua_22010_2")]
    TestLua22010_2,

    #[serde(rename = "TEST/Lua_22028_1")]
    TestLua22028_1,

    #[serde(rename = "TEST/Lua_22304_1")]
    TestLua22304_1,

    #[serde(rename = "TEST/Lua_22306_1")]
    TestLua22306_1,

    #[serde(rename = "TEST/Lua_22307_1")]
    TestLua22307_1,

    #[serde(rename = "TEST/Lua_465_7")]
    TestLua465_7,

    #[serde(rename = "TEST/Lua_71012_1")]
    TestLua71012_1,

    #[serde(rename = "TEST/Lua_72100_1")]
    TestLua72100_1,

    #[serde(rename = "TEST/Lua_72100_2")]
    TestLua72100_2,

    #[serde(rename = "TEST/Lua_72100_3")]
    TestLua72100_3,

    #[serde(rename = "TEST/Lua_72100_4")]
    TestLua72100_4,

    #[serde(rename = "TEST/Lua_72107_1")]
    TestLua72107_1,

    #[serde(rename = "TEST/Lua_72107_2")]
    TestLua72107_2,

    #[serde(rename = "TEST/Lua_72108_1")]
    TestLua72108_1,

    #[serde(rename = "TEST/Lua_72113_2")]
    TestLua72113_2,

    #[serde(rename = "TEST/Lua_72113_3")]
    TestLua72113_3,

    #[serde(rename = "TEST/Lua_72142_1")]
    TestLua72142_1,

    #[serde(rename = "TEST/Lua_72154_1")]
    TestLua72154_1,

    #[serde(rename = "TEST/RandomQuestLerpOutNpc")]
    TestRandomQuestLerpOutNpc,

    #[serde(rename = "TEST/TEST_GODDESS_02")]
    TestTestGoddess02,

    #[serde(rename = "UI/open_activity_arenachallengepage")]
    UiOpenActivityArenachallengepage,

    #[serde(rename = "UI/open_activity_bartender_make")]
    UiOpenActivityBartenderMake,

    #[serde(rename = "UI/open_activity_bartender_mission")]
    UiOpenActivityBartenderMission,

    #[serde(rename = "UI/open_activity_blessing_page")]
    UiOpenActivityBlessingPage,

    #[serde(rename = "UI/open_activity_channellerslab_mainpage")]
    UiOpenActivityChannellerslabMainpage,

    #[serde(rename = "UI/open_activity_channellerslab_page")]
    UiOpenActivityChannellerslabPage,

    #[serde(rename = "UI/open_activity_contribution_page")]
    UiOpenActivityContributionPage,

    #[serde(rename = "UI/open_activity_flightchallengepage")]
    UiOpenActivityFlightchallengepage,

    #[serde(rename = "UI/open_activity_fooddelivery_page")]
    UiOpenActivityFooddeliveryPage,

    #[serde(rename = "UI/open_activity_gacha_codex_page")]
    UiOpenActivityGachaCodexPage,

    #[serde(rename = "UI/open_activity_gacha_exchange_page")]
    UiOpenActivityGachaExchangePage,

    #[serde(rename = "UI/open_activity_gacha_forging_page")]
    UiOpenActivityGachaForgingPage,

    #[serde(rename = "UI/open_activity_GrowFlowers_page")]
    UiOpenActivityGrowFlowersPage,

    #[serde(rename = "UI/open_activity_LunaRiteMarkPage_Inazuma")]
    UiOpenActivityLunaRiteMarkPageInazuma,

    #[serde(rename = "UI/open_activity_LunaRiteMarkPage_Liyue")]
    UiOpenActivityLunaRiteMarkPageLiyue,

    #[serde(rename = "UI/open_activity_LunaRiteMarkPage_Mengde")]
    UiOpenActivityLunaRiteMarkPageMengde,

    #[serde(rename = "UI/open_activity_progress_page")]
    UiOpenActivityProgressPage,

    #[serde(rename = "UI/open_activity_treasurehuntpage")]
    UiOpenActivityTreasurehuntpage,

    #[serde(rename = "UI/open_activity_vintage_marketdeliver_30235")]
    UiOpenActivityVintageMarketdeliver30235,

    #[serde(rename = "UI/open_activity_vintage_marketdeliver_30236")]
    UiOpenActivityVintageMarketdeliver30236,

    #[serde(rename = "UI/open_activity_vintage_marketmainpage_30234")]
    UiOpenActivityVintageMarketmainpage30234,

    #[serde(rename = "UI/open_activity_vintage_marketmainpage_30243")]
    UiOpenActivityVintageMarketmainpage30243,

    #[serde(rename = "UI/open_activity_vintage_marketmainpage_30244")]
    UiOpenActivityVintageMarketmainpage30244,

    #[serde(rename = "UI/open_atreMechanicus_page")]
    UiOpenAtreMechanicusPage,

    #[serde(rename = "UI/open_bigmap_page")]
    UiOpenBigmapPage,

    #[serde(rename = "UI/open_drawlots_page")]
    UiOpenDrawlotsPage,

    #[serde(rename = "UI/open_explorationpage")]
    UiOpenExplorationpage,

    #[serde(rename = "UI/open_explorationpage_02")]
    UiOpenExplorationpage02,

    #[serde(rename = "UI/open_forging_page")]
    UiOpenForgingPage,

    #[serde(rename = "UI/open_giving_page")]
    UiOpenGivingPage,

    #[serde(rename = "UI/open_goddess_page")]
    UiOpenGoddessPage,

    #[serde(rename = "UI/open_homeworld_build_page_visitor")]
    UiOpenHomeworldBuildPageVisitor,

    #[serde(rename = "UI/open_homeworld_limited_shop_page")]
    UiOpenHomeworldLimitedShopPage,

    #[serde(rename = "UI/open_luminance_stone_page")]
    UiOpenLuminanceStonePage,

    #[serde(rename = "UI/open_playerlevel_page")]
    UiOpenPlayerlevelPage,

    #[serde(rename = "UI/open_playerlevel_page_Inazuma")]
    UiOpenPlayerlevelPageInazuma,

    #[serde(rename = "UI/open_playerlevel_page_Liyue")]
    UiOpenPlayerlevelPageLiyue,

    #[serde(rename = "UI/open_playerlevel_page_Sumeru")]
    UiOpenPlayerlevelPageSumeru,

    #[serde(rename = "UI/open_reputation_page")]
    UiOpenReputationPage,

    #[serde(rename = "UI/open_spring_use_page")]
    UiOpenSpringUsePage,

    #[serde(rename = "UI/open_synthesis_page")]
    UiOpenSynthesisPage,

    #[serde(rename = "UI/open_teampage")]
    UiOpenTeampage,

    #[serde(rename = "UI/open_violin_repair2_page")]
    UiOpenViolinRepair2Page,

    #[serde(rename = "UI/open_violin_repair_page")]
    UiOpenViolinRepairPage,
}

#[derive(Serialize, Deserialize)]
pub enum ActionBefore {
    #[serde(rename = "default_empty")]
    DefaultEmpty,

    #[serde(rename = "DialogAction/D220410303_Before")]
    DialogActionD220410303Before,

    #[serde(rename = "DialogAction/D230260204_After")]
    DialogActionD230260204After,

    #[serde(rename = "DialogAction/D230260206_After")]
    DialogActionD230260206After,

    #[serde(rename = "DialogAction/D230430129_Before")]
    DialogActionD230430129Before,

    #[serde(rename = "DialogAction/D230430130_Before")]
    DialogActionD230430130Before,

    #[serde(rename = "DialogAction/D230431702_Before")]
    DialogActionD230431702Before,

    #[serde(rename = "DialogAction/D400880401_While")]
    DialogActionD400880401While,

    #[serde(rename = "DialogAction/D710600101_After")]
    DialogActionD710600101After,

    #[serde(rename = "DialogAction/D710790102_Before")]
    DialogActionD710790102Before,

    #[serde(rename = "DialogAction/D710790202_Before")]
    DialogActionD710790202Before,

    #[serde(rename = "DialogAction/D710790302_Before")]
    DialogActionD710790302Before,

    #[serde(rename = "DialogAction/D710790402_Before")]
    DialogActionD710790402Before,

    #[serde(rename = "DialogAction/D710790502_Before")]
    DialogActionD710790502Before,

    #[serde(rename = "DialogAction/D710790602_Before")]
    DialogActionD710790602Before,

    #[serde(rename = "DialogAction/D710790702_Before")]
    DialogActionD710790702Before,

    #[serde(rename = "DialogAction/D710790802_Before")]
    DialogActionD710790802Before,

    #[serde(rename = "DialogAction/D710790902_Before")]
    DialogActionD710790902Before,

    #[serde(rename = "DialogAction/D710791002_Before")]
    DialogActionD710791002Before,

    #[serde(rename = "DialogAction/D710791102_Before")]
    DialogActionD710791102Before,

    #[serde(rename = "DialogAction/D710791202_Before")]
    DialogActionD710791202Before,

    #[serde(rename = "DialogAction/D721490201_Before.json")]
    DialogActionD721490201BeforeJson,

    #[serde(rename = "DialogAction/D721490301_Before.json")]
    DialogActionD721490301BeforeJson,

    #[serde(rename = "DialogAction/D721510101_Before.json")]
    DialogActionD721510101BeforeJson,

    #[serde(rename = "DialogAction/D721510401_Before.json")]
    DialogActionD721510401BeforeJson,

    #[serde(rename = "DialogAction/D721510701_Before.json")]
    DialogActionD721510701BeforeJson,

    #[serde(rename = "")]
    Empty,

    #[serde(rename = "HomeWorld/huling_bansleep")]
    HomeWorldHulingBansleep,

    #[serde(rename = "HomeWorld/huling_start")]
    HomeWorldHulingStart,

    #[serde(rename = "QuestDialogue/IQ/Inazuma_22014/Q2201401")]
    QuestDialogueIqInazuma22014Q2201401,

    #[serde(rename = "SimpleTalk/Akimbo")]
    SimpleTalkAkimbo,

    #[serde(rename = "SimpleTalk/Akimbo_Start")]
    SimpleTalkAkimboStart,

    #[serde(rename = "SimpleTalk/ChildCoverEye")]
    SimpleTalkChildCoverEye,

    #[serde(rename = "SimpleTalk/Confuse")]
    SimpleTalkConfuse,

    #[serde(rename = "SimpleTalk/Confuse_Start")]
    SimpleTalkConfuseStart,

    #[serde(rename = "SimpleTalk/Downcast")]
    SimpleTalkDowncast,

    #[serde(rename = "SimpleTalk/Downcast_Start")]
    SimpleTalkDowncastStart,

    #[serde(rename = "SimpleTalk/Greet_Start")]
    SimpleTalkGreetStart,

    #[serde(rename = "SimpleTalk/HoldArm")]
    SimpleTalkHoldArm,

    #[serde(rename = "SimpleTalk/HoldHead")]
    SimpleTalkHoldHead,

    #[serde(rename = "SimpleTalk/HoldHead_Start")]
    SimpleTalkHoldHeadStart,

    #[serde(rename = "SimpleTalk/LerpInNpcLookAtOnly_Start")]
    SimpleTalkLerpInNpcLookAtOnlyStart,

    #[serde(rename = "SimpleTalk/LerpInNpcNoSteer_Start")]
    SimpleTalkLerpInNpcNoSteerStart,

    #[serde(rename = "SimpleTalk/PutHand")]
    SimpleTalkPutHand,

    #[serde(rename = "SimpleTalk/RandomTalk")]
    SimpleTalkRandomTalk,

    #[serde(rename = "SimpleTalk/RandomTalkHold_Start")]
    SimpleTalkRandomTalkHoldStart,

    #[serde(rename = "SimpleTalk/RandomTalk_Start")]
    SimpleTalkRandomTalkStart,

    #[serde(rename = "SimpleTalk/Request01_Start")]
    SimpleTalkRequest01Start,

    #[serde(rename = "SimpleTalk/ShakeHead")]
    SimpleTalkShakeHead,

    #[serde(rename = "SimpleTalk/Standby")]
    SimpleTalkStandby,

    #[serde(rename = "SimpleTalk/StrikeChest")]
    SimpleTalkStrikeChest,

    #[serde(rename = "SimpleTalk/StrikeChest_Start")]
    SimpleTalkStrikeChestStart,

    #[serde(rename = "SimpleTalk/Think_Start")]
    SimpleTalkThinkStart,

    #[serde(rename = "TEST/CutToPaimon1")]
    TestCutToPaimon1,

    #[serde(rename = "TEST/EmojiBubble_Insight")]
    TestEmojiBubbleInsight,

    #[serde(rename = "TEST/EmojiBubble_Thinking")]
    TestEmojiBubbleThinking,

    #[serde(rename = "TEST/FadeToBlack")]
    TestFadeToBlack,

    #[serde(rename = "TEST/LerpInLady")]
    TestLerpInLady,

    #[serde(rename = "TEST/LerpInMale")]
    TestLerpInMale,

    #[serde(rename = "TEST/LerpInNpc")]
    TestLerpInNpc,

    #[serde(rename = "TEST/LerpInNpcExitFreestyle")]
    TestLerpInNpcExitFreestyle,

    #[serde(rename = "TEST/LerpInNpcExitFreestyleDelay")]
    TestLerpInNpcExitFreestyleDelay,

    #[serde(rename = "TEST/LerpInNpcLookAtOnly")]
    TestLerpInNpcLookAtOnly,

    #[serde(rename = "TEST/LerpInNpcNoSteer")]
    TestLerpInNpcNoSteer,

    #[serde(rename = "TEST/LerpInNpcSteer")]
    TestLerpInNpcSteer,

    #[serde(rename = "TEST/LerpInPaimon")]
    TestLerpInPaimon,

    #[serde(rename = "TEST/LerpInPaimonFirstCutscene")]
    TestLerpInPaimonFirstCutscene,

    #[serde(rename = "TEST/LerpInPaimon_TalkTest")]
    TestLerpInPaimonTalkTest,

    #[serde(rename = "TEST/LerpInQueen")]
    TestLerpInQueen,

    #[serde(rename = "TEST/LerpInQueenDefault")]
    TestLerpInQueenDefault,

    #[serde(rename = "TEST/LerplnNpcSteer")]
    TestLerplnNpcSteer,

    #[serde(rename = "TEST/Lua_10112_1")]
    TestLua10112_1,

    #[serde(rename = "TEST/Lua_10112_2")]
    TestLua10112_2,

    #[serde(rename = "TEST/Lua_10112_3")]
    TestLua10112_3,

    #[serde(rename = "TEST/Lua_20037_1")]
    TestLua20037_1,

    #[serde(rename = "TEST/Lua_20037_2")]
    TestLua20037_2,

    #[serde(rename = "TEST/Lua_20043_1")]
    TestLua20043_1,

    #[serde(rename = "TEST/Lua_20043_2")]
    TestLua20043_2,

    #[serde(rename = "TEST/Lua_20043_3")]
    TestLua20043_3,

    #[serde(rename = "TEST/Lua_20043_4")]
    TestLua20043_4,

    #[serde(rename = "TEST/Lua_20058_1")]
    TestLua20058_1,

    #[serde(rename = "TEST/Lua_22027_1")]
    TestLua22027_1,

    #[serde(rename = "TEST/Lua_22116_1")]
    TestLua22116_1,

    #[serde(rename = "TEST/Lua_22302_1")]
    TestLua22302_1,

    #[serde(rename = "TEST/Readable01")]
    TestReadable01,

    #[serde(rename = "TEST/TestTalk4")]
    TestTestTalk4,

    #[serde(rename = "UI/open_homeworld_build_page")]
    UiOpenHomeworldBuildPage,

    #[serde(rename = "UI/open_homeworld_level_page")]
    UiOpenHomeworldLevelPage,

    #[serde(rename = "UI/open_homeworld_shop_page")]
    UiOpenHomeworldShopPage,

    #[serde(rename = "UI/open_homeworld_switch_page")]
    UiOpenHomeworldSwitchPage,
}

#[derive(Serialize, Deserialize)]
pub enum ActionWhile {
    #[serde(rename = "DialogAction/D200030101_While")]
    DialogActionD200030101While,

    #[serde(rename = "DialogAction/D200030104_While")]
    DialogActionD200030104While,

    #[serde(rename = "DialogAction/D200030113_While")]
    DialogActionD200030113While,

    #[serde(rename = "DialogAction/D200030301_While")]
    DialogActionD200030301While,

    #[serde(rename = "DialogAction/D200030302_While")]
    DialogActionD200030302While,

    #[serde(rename = "DialogAction/D200290101_While")]
    DialogActionD200290101While,

    #[serde(rename = "DialogAction/D200290703_While")]
    DialogActionD200290703While,

    #[serde(rename = "DialogAction/D230250601_While")]
    DialogActionD230250601While,

    #[serde(rename = "DialogAction/D230250901_While")]
    DialogActionD230250901While,

    #[serde(rename = "DialogAction/D230430201_While")]
    DialogActionD230430201While,

    #[serde(rename = "DialogAction/D230430507_While")]
    DialogActionD230430507While,

    #[serde(rename = "DialogAction/D230430509_While")]
    DialogActionD230430509While,

    #[serde(rename = "DialogAction/D230430908_While")]
    DialogActionD230430908While,

    #[serde(rename = "DialogAction/D230430910_While")]
    DialogActionD230430910While,

    #[serde(rename = "DialogAction/D230431604_While")]
    DialogActionD230431604While,

    #[serde(rename = "DialogAction/D400881401_While")]
    DialogActionD400881401While,

    #[serde(rename = "")]
    Empty,

    #[serde(rename = "HomeWorld/huling_start")]
    HomeWorldHulingStart,

    #[serde(rename = "Quest_MQ354_ShowFeather")]
    QuestMq354ShowFeather,

    #[serde(rename = "rt")]
    Rt,

    #[serde(rename = "SimpleTalk/Agree")]
    SimpleTalkAgree,

    #[serde(rename = "SimpleTalk/Akimbo")]
    SimpleTalkAkimbo,

    #[serde(rename = "SimpleTalk/Akimbo_Start")]
    SimpleTalkAkimboStart,

    #[serde(rename = "SimpleTalk/CLap_Start")]
    SimpleTalkCLapStart,

    #[serde(rename = "SimpleTalk/ChildCoverEye")]
    SimpleTalkChildCoverEye,

    #[serde(rename = "SimpleTalk/Clap")]
    SimpleTalkClap,

    #[serde(rename = "SimpleTalk/Confuse")]
    SimpleTalkConfuse,

    #[serde(rename = "SimpleTalk/Confuse_Start")]
    SimpleTalkConfuseStart,

    #[serde(rename = "SimpleTalk/Downcast")]
    SimpleTalkDowncast,

    #[serde(rename = "SimpleTalk/Downcast_Start")]
    SimpleTalkDowncastStart,

    #[serde(rename = "SimpleTalk/Foreke")]
    SimpleTalkForeke,

    #[serde(rename = "SimpleTalk/Forerke")]
    SimpleTalkForerke,

    #[serde(rename = "SimpleTalk/Forerke_Start")]
    SimpleTalkForerkeStart,

    #[serde(rename = "SimpleTalk/Greet")]
    SimpleTalkGreet,

    #[serde(rename = "SimpleTalk/Greet_Start")]
    SimpleTalkGreetStart,

    #[serde(rename = "SimpleTalk/Happy")]
    SimpleTalkHappy,

    #[serde(rename = "SimpleTalk/HoldAram")]
    SimpleTalkHoldAram,

    #[serde(rename = "SimpleTalk/HoldAram_Start")]
    SimpleTalkHoldAramStart,

    #[serde(rename = "SimpleTalk/HoldArm")]
    SimpleTalkHoldArm,

    #[serde(rename = "SimpleTalk/HoldArm_Start")]
    SimpleTalkHoldArmStart,

    #[serde(rename = "SimpleTalk/HoldHead")]
    SimpleTalkHoldHead,

    #[serde(rename = "SimpleTalk/HoldHead_Start")]
    SimpleTalkHoldHeadStart,

    #[serde(rename = "SimpleTalk/Idle_End")]
    SimpleTalkIdleEnd,

    #[serde(rename = "SimpleTalk/LerpInNpcDailyTalk_Start")]
    SimpleTalkLerpInNpcDailyTalkStart,

    #[serde(rename = "SimpleTalk/LerpInNpcNoSteer_Start")]
    SimpleTalkLerpInNpcNoSteerStart,

    #[serde(rename = "SimpleTalk/NodHead")]
    SimpleTalkNodHead,

    #[serde(rename = "SimpleTalk/NpcDailyTalkRandomFS")]
    SimpleTalkNpcDailyTalkRandomFs,

    #[serde(rename = "SimpleTalk/PutHand")]
    SimpleTalkPutHand,

    #[serde(rename = "SimpleTalk/RandomTalk")]
    SimpleTalkRandomTalk,

    #[serde(rename = "SimpleTalk/RandomTalkHold")]
    SimpleTalkRandomTalkHold,

    #[serde(rename = "SimpleTalk/RandomTalkHold_Start")]
    SimpleTalkRandomTalkHoldStart,

    #[serde(rename = "SimpleTalk/RandomTalk_Start")]
    SimpleTalkRandomTalkStart,

    #[serde(rename = "SimpleTalk/Refuse")]
    SimpleTalkRefuse,

    #[serde(rename = "SimpleTalk/Refuse_Start")]
    SimpleTalkRefuseStart,

    #[serde(rename = "SimpleTalk/Request01")]
    SimpleTalkRequest01,

    #[serde(rename = "SimpleTalk/Request01_Start")]
    SimpleTalkRequest01Start,

    #[serde(rename = "SimpleTalk/ShakeHead")]
    SimpleTalkShakeHead,

    #[serde(rename = "SimpleTalk/Shrug")]
    SimpleTalkShrug,

    #[serde(rename = "SimpleTalk/Shrug_Start")]
    SimpleTalkShrugStart,

    #[serde(rename = "SimpleTalk/Shy")]
    SimpleTalkShy,

    #[serde(rename = "SimpleTalk/Shy_Start")]
    SimpleTalkShyStart,

    #[serde(rename = "SimpleTalk/Sit01_Start")]
    SimpleTalkSit01Start,

    #[serde(rename = "SimpleTalk/SitHoldArm")]
    SimpleTalkSitHoldArm,

    #[serde(rename = "SimpleTalk/SitHoldArmHand")]
    SimpleTalkSitHoldArmHand,

    #[serde(rename = "SimpleTalk/SitHoldArm_Start")]
    SimpleTalkSitHoldArmStart,

    #[serde(rename = "SimpleTalk/SitPutArm")]
    SimpleTalkSitPutArm,

    #[serde(rename = "SimpleTalk/SitPutArm_Start")]
    SimpleTalkSitPutArmStart,

    #[serde(rename = "SimpleTalk/Standby")]
    SimpleTalkStandby,

    #[serde(rename = "SimpleTalk/SteerOnly")]
    SimpleTalkSteerOnly,

    #[serde(rename = "SimpleTalk/StrikeChest")]
    SimpleTalkStrikeChest,

    #[serde(rename = "SimpleTalk/StrikeChest_Start")]
    SimpleTalkStrikeChestStart,

    #[serde(rename = "SimpleTalk/Think")]
    SimpleTalkThink,

    #[serde(rename = "SimpleTalk/Think_Start")]
    SimpleTalkThinkStart,

    #[serde(rename = "TEST/BodyLang_Guard")]
    TestBodyLangGuard,

    #[serde(rename = "TEST/BodyLang_Standby")]
    TestBodyLangStandby,

    #[serde(rename = "TEST/D200490101")]
    TestD200490101,

    #[serde(rename = "TEST/D700080113")]
    TestD700080113,

    #[serde(rename = "TEST/D700080117")]
    TestD700080117,

    #[serde(rename = "TEST/EchoConchShowEff")]
    TestEchoConchShowEff,

    #[serde(rename = "TEST/EmojiBubble_Afraid")]
    TestEmojiBubbleAfraid,

    #[serde(rename = "TEST/EmojiBubble_Angry")]
    TestEmojiBubbleAngry,

    #[serde(rename = "TEST/EmojiBubble_Awkward")]
    TestEmojiBubbleAwkward,

    #[serde(rename = "TEST/EmojiBubble_Exciting")]
    TestEmojiBubbleExciting,

    #[serde(rename = "TEST/EmojiBubble_Happy")]
    TestEmojiBubbleHappy,

    #[serde(rename = "TEST/EmojiBubble_Insight")]
    TestEmojiBubbleInsight,

    #[serde(rename = "TEST/EmojiBubble_Love")]
    TestEmojiBubbleLove,

    #[serde(rename = "TEST/EmojiBubble_Query")]
    TestEmojiBubbleQuery,

    #[serde(rename = "TEST/EmojiBubble_Sad")]
    TestEmojiBubbleSad,

    #[serde(rename = "TEST/EmojiBubble_Surprise")]
    TestEmojiBubbleSurprise,

    #[serde(rename = "TEST/EmojiBubble_Sweat")]
    TestEmojiBubbleSweat,

    #[serde(rename = "TEST/EmojiBubble_Thinking")]
    TestEmojiBubbleThinking,

    #[serde(rename = "TEST/EmojiBubble_Worry")]
    TestEmojiBubbleWorry,

    #[serde(rename = "TEST/LerpInNpc")]
    TestLerpInNpc,

    #[serde(rename = "TEST/LerpInNpcLookAtOnly")]
    TestLerpInNpcLookAtOnly,

    #[serde(rename = "TEST/LerpInNpcNoSteer")]
    TestLerpInNpcNoSteer,

    #[serde(rename = "TEST/LerpOutNpcNoSteer")]
    TestLerpOutNpcNoSteer,

    #[serde(rename = "TEST/Lua_10100_5")]
    TestLua10100_5,

    #[serde(rename = "TEST/Lua_10100_6")]
    TestLua10100_6,

    #[serde(rename = "TEST/Lua_10100_7")]
    TestLua10100_7,

    #[serde(rename = "TEST/Lua_10501_1")]
    TestLua10501_1,

    #[serde(rename = "TEST/Lua_10501_2")]
    TestLua10501_2,

    #[serde(rename = "TEST/Lua_10501_3")]
    TestLua10501_3,

    #[serde(rename = "TEST/Lua_10501_4")]
    TestLua10501_4,

    #[serde(rename = "TEST/Lua_11101_1")]
    TestLua11101_1,

    #[serde(rename = "TEST/Lua_11101_2")]
    TestLua11101_2,

    #[serde(rename = "TEST/Lua_11101_3")]
    TestLua11101_3,

    #[serde(rename = "TEST/Lua_20040_1")]
    TestLua20040_1,

    #[serde(rename = "TEST/Lua_20040_2")]
    TestLua20040_2,

    #[serde(rename = "TEST/Lua_20040_3")]
    TestLua20040_3,

    #[serde(rename = "TEST/Lua_20051_1")]
    TestLua20051_1,

    #[serde(rename = "TEST/Lua_20054_1")]
    TestLua20054_1,

    #[serde(rename = "TEST/Lua_20054_2")]
    TestLua20054_2,

    #[serde(rename = "TEST/Lua_20062_1")]
    TestLua20062_1,

    #[serde(rename = "TEST/Lua_20062_2")]
    TestLua20062_2,

    #[serde(rename = "TEST/Lua_20062_3")]
    TestLua20062_3,

    #[serde(rename = "TEST/Lua_20062_4")]
    TestLua20062_4,

    #[serde(rename = "TEST/Lua_20101_1")]
    TestLua20101_1,

    #[serde(rename = "TEST/Lua_20101_11")]
    TestLua20101_11,

    #[serde(rename = "TEST/Lua_20101_11A")]
    TestLua20101_11A,

    #[serde(rename = "TEST/Lua_20101_11B")]
    TestLua20101_11B,

    #[serde(rename = "TEST/Lua_20101_11C")]
    TestLua20101_11C,

    #[serde(rename = "TEST/Lua_20101_11X")]
    TestLua20101_11X,

    #[serde(rename = "TEST/Lua_20101_12")]
    TestLua20101_12,

    #[serde(rename = "TEST/Lua_20101_13")]
    TestLua20101_13,

    #[serde(rename = "TEST/Lua_20101_1A")]
    TestLua20101_1A,

    #[serde(rename = "TEST/Lua_20101_1B")]
    TestLua20101_1B,

    #[serde(rename = "TEST/Lua_20101_1C")]
    TestLua20101_1C,

    #[serde(rename = "TEST/Lua_20101_1X")]
    TestLua20101_1X,

    #[serde(rename = "TEST/Lua_20101_2")]
    TestLua20101_2,

    #[serde(rename = "TEST/Lua_20101_21")]
    TestLua20101_21,

    #[serde(rename = "TEST/Lua_20101_21A")]
    TestLua20101_21A,

    #[serde(rename = "TEST/Lua_20101_21B")]
    TestLua20101_21B,

    #[serde(rename = "TEST/Lua_20101_21C")]
    TestLua20101_21C,

    #[serde(rename = "TEST/Lua_20101_21X")]
    TestLua20101_21X,

    #[serde(rename = "TEST/Lua_20101_22")]
    TestLua20101_22,

    #[serde(rename = "TEST/Lua_20101_23")]
    TestLua20101_23,

    #[serde(rename = "TEST/Lua_20101_3")]
    TestLua20101_3,

    #[serde(rename = "TEST/Lua_20103_1")]
    TestLua20103_1,

    #[serde(rename = "TEST/Lua_20517_1")]
    TestLua20517_1,

    #[serde(rename = "TEST/Lua_22004_1")]
    TestLua22004_1,

    #[serde(rename = "TEST/Lua_22004_2")]
    TestLua22004_2,

    #[serde(rename = "TEST/Lua_22021_1")]
    TestLua22021_1,

    #[serde(rename = "TEST/Lua_22021_2")]
    TestLua22021_2,

    #[serde(rename = "TEST/Lua_22106_1")]
    TestLua22106_1,

    #[serde(rename = "TEST/Lua_22106_2")]
    TestLua22106_2,

    #[serde(rename = "TEST/Lua_22107_1")]
    TestLua22107_1,

    #[serde(rename = "TEST/Lua_22107_2")]
    TestLua22107_2,

    #[serde(rename = "TEST/Lua_22107_3")]
    TestLua22107_3,

    #[serde(rename = "TEST/Lua_22107_4")]
    TestLua22107_4,

    #[serde(rename = "TEST/Lua_22107_5")]
    TestLua22107_5,

    #[serde(rename = "TEST/Lua_22115_1")]
    TestLua22115_1,

    #[serde(rename = "TEST/Lua_41125_1")]
    TestLua41125_1,

    #[serde(rename = "TEST/Lua_41125_2")]
    TestLua41125_2,

    #[serde(rename = "TEST/Lua_41125_3")]
    TestLua41125_3,

    #[serde(rename = "TEST/Lua_41127_1")]
    TestLua41127_1,

    #[serde(rename = "TEST/Lua_41127_2")]
    TestLua41127_2,

    #[serde(rename = "TEST/Lua_41127_3")]
    TestLua41127_3,

    #[serde(rename = "TEST/Lua_426_1")]
    TestLua426_1,

    #[serde(rename = "TEST/Lua_464_3")]
    TestLua464_3,

    #[serde(rename = "TEST/Lua_465_10")]
    TestLua465_10,

    #[serde(rename = "TEST/Lua_466_1")]
    TestLua466_1,

    #[serde(rename = "TEST/Lua_466_2")]
    TestLua466_2,

    #[serde(rename = "TEST/Lua_466_5")]
    TestLua466_5,

    #[serde(rename = "TEST/Lua_466_6")]
    TestLua466_6,

    #[serde(rename = "TEST/Lua_466_7")]
    TestLua466_7,

    #[serde(rename = "TEST/Lua_466_8")]
    TestLua466_8,

    #[serde(rename = "TEST/Lua_467_1")]
    TestLua467_1,

    #[serde(rename = "TEST/Lua_487")]
    TestLua487,

    #[serde(rename = "TEST/Lua_70144_1")]
    TestLua70144_1,

    #[serde(rename = "TEST/Lua_71008_1")]
    TestLua71008_1,

    #[serde(rename = "TEST/Lua_71008_2")]
    TestLua71008_2,

    #[serde(rename = "TEST/Lua_71008_3")]
    TestLua71008_3,

    #[serde(rename = "TEST/Lua_71008_4")]
    TestLua71008_4,

    #[serde(rename = "TEST/Lua_71008_5")]
    TestLua71008_5,

    #[serde(rename = "TEST/RandomQuestLerpInNpc")]
    TestRandomQuestLerpInNpc,

    #[serde(rename = "TEST/TEST_Chest_Btn")]
    TestTestChestBtn,

    #[serde(rename = "UI/open_homeworld_level_page")]
    UiOpenHomeworldLevelPage,
}

#[derive(Serialize, Deserialize)]
pub enum OptionIcon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Icon_Intee_ActivityProps")]
    UiIconInteeActivityProps,

    #[serde(rename = "UI_Icon_Intee_AreaUpgrad")]
    UiIconInteeAreaUpgrad,

    #[serde(rename = "UI_Icon_Intee_Astrology")]
    UiIconInteeAstrology,

    #[serde(rename = "UI_Icon_Intee_Blacksmith")]
    UiIconInteeBlacksmith,

    #[serde(rename = "UI_Icon_Intee_Blessing")]
    UiIconInteeBlessing,

    #[serde(rename = "UI_Icon_Intee_Comfort")]
    UiIconInteeComfort,

    #[serde(rename = "UI_Icon_Intee_DailyEvent_0")]
    UiIconInteeDailyEvent0,

    #[serde(rename = "UI_Icon_Intee_DailyEvent_1")]
    UiIconInteeDailyEvent1,

    #[serde(rename = "UI_Icon_Intee_Explore_0")]
    UiIconInteeExplore0,

    #[serde(rename = "UI_Icon_Intee_Fishes")]
    UiIconInteeFishes,

    #[serde(rename = "UI_Icon_Intee_FlightChallenge")]
    UiIconInteeFlightChallenge,

    #[serde(rename = "UI_Icon_Intee_FurnitureBuild")]
    UiIconInteeFurnitureBuild,

    #[serde(rename = "UI_Icon_Intee_GeneralCargo")]
    UiIconInteeGeneralCargo,

    #[serde(rename = "UI_Icon_Intee_GrowFlowers")]
    UiIconInteeGrowFlowers,

    #[serde(rename = "UI_Icon_Intee_HomeSwitch")]
    UiIconInteeHomeSwitch,

    #[serde(rename = "UI_Icon_Intee_LuminanceStone")]
    UiIconInteeLuminanceStone,

    #[serde(rename = "UI_Icon_Intee_Mechanism")]
    UiIconInteeMechanism,

    #[serde(rename = "UI_Icon_Intee_Miscsmarvs")]
    UiIconInteeMiscsmarvs,

    #[serde(rename = "UI_Icon_Intee_PlayerLevel_0")]
    UiIconInteePlayerLevel0,

    #[serde(rename = "UI_Icon_Intee_Reputation")]
    UiIconInteeReputation,

    #[serde(rename = "UI_Icon_Intee_Restaurant")]
    UiIconInteeRestaurant,

    #[serde(rename = "UI_Icon_Intee_Shop")]
    UiIconInteeShop,

    #[serde(rename = "UI_Icon_Intee_Souvenir")]
    UiIconInteeSouvenir,

    #[serde(rename = "UI_Icon_Intee_Speedup")]
    UiIconInteeSpeedup,

    #[serde(rename = "UI_Icon_Intee_TalkSpecial")]
    UiIconInteeTalkSpecial,

    #[serde(rename = "UI_Icon_Intee_TheatreMechanicus")]
    UiIconInteeTheatreMechanicus,

    #[serde(rename = "UI_Icon_Intee_TreasureBox")]
    UiIconInteeTreasureBox,

    #[serde(rename = "UI_Icon_Intee_TreasureGift")]
    UiIconInteeTreasureGift,

    #[serde(rename = "UI_Icon_Intee_TreasureHunt")]
    UiIconInteeTreasureHunt,

    #[serde(rename = "UI_Icon_Intee_Vintage_01")]
    UiIconInteeVintage01,

    #[serde(rename = "UI_Icon_Intee_Vintage_02")]
    UiIconInteeVintage02,

    #[serde(rename = "UI_Icon_Intee_Vintage_03")]
    UiIconInteeVintage03,

    #[serde(rename = "UI_Icon_Quest_Once")]
    UiIconQuestOnce,

    #[serde(rename = "UI_NPCTopIcon_Activity_Music")]
    UiNpcTopIconActivityMusic,

    #[serde(rename = "UI_NPCTopIcon_Activity_ProjectionNPC")]
    UiNpcTopIconActivityProjectionNpc,

    #[serde(rename = "UI_NPCTopIcon_Activity_RobotGacha")]
    UiNpcTopIconActivityRobotGacha,

    #[serde(rename = "UI_NPCTopIcon_Blacksmith")]
    UiNpcTopIconBlacksmith,

    #[serde(rename = "UI_NPCTopIcon_Combine")]
    UiNpcTopIconCombine,

    #[serde(rename = "UI_NPCTopIcon_EditTeam")]
    UiNpcTopIconEditTeam,

    #[serde(rename = "UI_NPCTopIcon_MiscsMarvs")]
    UiNpcTopIconMiscsMarvs,

    #[serde(rename = "UI_NPCTopIcon_Restaurant")]
    UiNpcTopIconRestaurant,

    #[serde(rename = "UI_NPCTopIcon_Souvenir")]
    UiNpcTopIconSouvenir,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "TALK_ROLE_BLACK_SCREEN")]
    TalkRoleBlackScreen,

    #[serde(rename = "TALK_ROLE_GADGET")]
    TalkRoleGadget,

    #[serde(rename = "TALK_ROLE_MATE_AVATAR")]
    TalkRoleMateAvatar,

    #[serde(rename = "TALK_ROLE_NEED_CLICK_BLACK_SCREEN")]
    TalkRoleNeedClickBlackScreen,

    #[serde(rename = "TALK_ROLE_NPC")]
    TalkRoleNpc,

    #[serde(rename = "TALK_ROLE_PLAYER")]
    TalkRolePlayer,
}

#[derive(Serialize, Deserialize)]
pub enum TalkShowType {
    #[serde(rename = "TALK_SHOW_FORCE_SELECT")]
    TalkShowForceSelect,
}
