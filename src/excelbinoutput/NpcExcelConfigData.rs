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

pub type NpcExcelConfigData = Vec<NpcExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NpcExcelConfigDatum {
    #[serde(rename = "jsonName")]
    pub json_name: JsonName,

    #[serde(rename = "alias")]
    pub alias: String,

    #[serde(rename = "scriptDataPath")]
    pub script_data_path: ScriptDataPath,

    #[serde(rename = "luaDataPath")]
    pub lua_data_path: LuaDataPath,

    #[serde(rename = "dyePart")]
    pub dye_part: String,

    #[serde(rename = "billboardIcon")]
    pub billboard_icon: BillboardIcon,

    #[serde(rename = "templateEmotionPath")]
    pub template_emotion_path: TemplateEmotionPath,

    #[serde(rename = "MEGNJOPONPJ")]
    pub megnjoponpj: Vec<i64>,

    #[serde(rename = "PAAANBHPCNL")]
    pub paaanbhpcnl: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "prefabPathHashSuffix")]
    pub prefab_path_hash_suffix: i64,

    #[serde(rename = "prefabPathHashPre")]
    pub prefab_path_hash_pre: i64,

    #[serde(rename = "campID")]
    pub camp_id: i64,

    #[serde(rename = "LODPatternName")]
    pub lod_pattern_name: String,

    #[serde(rename = "hasMove")]
    pub has_move: Option<bool>,

    #[serde(rename = "hasAudio")]
    pub has_audio: Option<bool>,

    #[serde(rename = "jsonPathHashSuffix")]
    pub json_path_hash_suffix: Option<i64>,

    #[serde(rename = "jsonPathHashPre")]
    pub json_path_hash_pre: Option<i64>,

    #[serde(rename = "isDaily")]
    pub is_daily: Option<bool>,

    #[serde(rename = "gachaCardNameHashSuffix")]
    pub gacha_card_name_hash_suffix: Option<i64>,

    #[serde(rename = "coopPicNameHashSuffix")]
    pub coop_pic_name_hash_suffix: Option<i64>,

    #[serde(rename = "invisiable")]
    pub invisiable: Option<bool>,

    #[serde(rename = "disableShowName")]
    pub disable_show_name: Option<bool>,

    #[serde(rename = "bodyType")]
    pub body_type: Option<BodyType>,

    #[serde(rename = "firstMetId")]
    pub first_met_id: Option<i64>,

    #[serde(rename = "KDOFFPNPJFD")]
    pub kdoffpnpjfd: Option<bool>,

    #[serde(rename = "billboardType")]
    pub billboard_type: Option<BillboardType>,

    #[serde(rename = "animatorConfigPathHashSuffix")]
    pub animator_config_path_hash_suffix: Option<i64>,

    #[serde(rename = "NJIEKKLKLLD")]
    pub njiekklklld: Option<i64>,

    #[serde(rename = "luaDataIndex")]
    pub lua_data_index: Option<i64>,

    #[serde(rename = "animatorConfigPathHashPre")]
    pub animator_config_path_hash_pre: Option<bool>,

    #[serde(rename = "BEHJMAEBDNO")]
    pub behjmaebdno: Option<bool>,

    #[serde(rename = "FJMJMIGMJCL")]
    pub fjmjmigmjcl: Option<Fjmjmigmjcl>,
}

#[derive(Serialize, Deserialize)]
pub enum BillboardIcon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_NPCTopIcon_Activity_Bartender")]
    UiNpcTopIconActivityBartender,

    #[serde(rename = "UI_NPCTopIcon_Activity_ChannellerSlab")]
    UiNpcTopIconActivityChannellerSlab,

    #[serde(rename = "UI_NPCTopIcon_Activity_Drake_Primo_Rock")]
    UiNpcTopIconActivityDrakePrimoRock,

    #[serde(rename = "UI_NPCTopIcon_Activity_FleurFair")]
    UiNpcTopIconActivityFleurFair,

    #[serde(rename = "UI_NPCTopIcon_Activity_Gear")]
    UiNpcTopIconActivityGear,

    #[serde(rename = "UI_NPCTopIcon_Activity_GravenInnocence_Shop")]
    UiNpcTopIconActivityGravenInnocenceShop,

    #[serde(rename = "UI_NPCTopIcon_Activity_GrowFlowers")]
    UiNpcTopIconActivityGrowFlowers,

    #[serde(rename = "UI_NPCTopIcon_Activity_IrodoriFlower")]
    UiNpcTopIconActivityIrodoriFlower,

    #[serde(rename = "UI_NPCTopIcon_Activity_IrodoriMaster")]
    UiNpcTopIconActivityIrodoriMaster,

    #[serde(rename = "UI_NPCTopIcon_Activity_IrodoriPoetry")]
    UiNpcTopIconActivityIrodoriPoetry,

    #[serde(rename = "UI_NPCTopIcon_Activity_Music")]
    UiNpcTopIconActivityMusic,

    #[serde(rename = "UI_NPCTopIcon_Activity_ProjectionNPC")]
    UiNpcTopIconActivityProjectionNpc,

    #[serde(rename = "UI_NPCTopIcon_ActivityProps")]
    UiNpcTopIconActivityProps,

    #[serde(rename = "UI_NPCTopIcon_Activity_RobotGacha")]
    UiNpcTopIconActivityRobotGacha,

    #[serde(rename = "UI_NPCTopIcon_Activity_Vintage_01")]
    UiNpcTopIconActivityVintage01,

    #[serde(rename = "UI_NPCTopIcon_Activity_Vintage_03")]
    UiNpcTopIconActivityVintage03,

    #[serde(rename = "UI_NPCTopIcon_Activity_Vintage_04")]
    UiNpcTopIconActivityVintage04,

    #[serde(rename = "UI_NPCTopIcon_Adventurers")]
    UiNpcTopIconAdventurers,

    #[serde(rename = "UI_NPCTopIcon_AnimalCatch")]
    UiNpcTopIconAnimalCatch,

    #[serde(rename = "UI_NPCTopIcon_Astrology")]
    UiNpcTopIconAstrology,

    #[serde(rename = "UI_NPCTopIcon_Blacksmith")]
    UiNpcTopIconBlacksmith,

    #[serde(rename = "UI_NPCTopIcon_Blessing")]
    UiNpcTopIconBlessing,

    #[serde(rename = "UI_NPCTopIcon_Combine")]
    UiNpcTopIconCombine,

    #[serde(rename = "UI_NPCTopIcon_Fishes")]
    UiNpcTopIconFishes,

    #[serde(rename = "UI_NPCTopIcon_FurnitureShop")]
    UiNpcTopIconFurnitureShop,

    #[serde(rename = "UI_NPCTopIcon_GeneralCargo")]
    UiNpcTopIconGeneralCargo,

    #[serde(rename = "UI_NPCTopIcon_HideandSeek")]
    UiNpcTopIconHideandSeek,

    #[serde(rename = "UI_NPCTopIcon_LuminanceStone")]
    UiNpcTopIconLuminanceStone,

    #[serde(rename = "UI_NPCTopIcon_LunaRite")]
    UiNpcTopIconLunaRite,

    #[serde(rename = "UI_NPCTopIcon_MiscsMarvs")]
    UiNpcTopIconMiscsMarvs,

    #[serde(rename = "UI_NPCTopIcon_Reputation")]
    UiNpcTopIconReputation,

    #[serde(rename = "UI_NPCTopIcon_Restaurant")]
    UiNpcTopIconRestaurant,

    #[serde(rename = "UI_NPCTopIcon_Souvenir")]
    UiNpcTopIconSouvenir,

    #[serde(rename = "UI_NPCTopIcon_TheatreMechanicus")]
    UiNpcTopIconTheatreMechanicus,

    #[serde(rename = "UI_NPCTopIcon_TreasureHunt")]
    UiNpcTopIconTreasureHunt,
}

#[derive(Serialize, Deserialize)]
pub enum BillboardType {
    #[serde(rename = "Icon")]
    Icon,

    #[serde(rename = "Sneak")]
    Sneak,
}

#[derive(Serialize, Deserialize)]
pub enum BodyType {
    #[serde(rename = "AVATAR_BOY")]
    AvatarBoy,

    #[serde(rename = "AVATAR_GIRL")]
    AvatarGirl,

    #[serde(rename = "AVATAR_LADY")]
    AvatarLady,

    #[serde(rename = "AVATAR_LOLI")]
    AvatarLoli,

    #[serde(rename = "AVATAR_MALE")]
    AvatarMale,

    #[serde(rename = "AVATAR_PAIMON")]
    AvatarPaimon,

    #[serde(rename = "Barbara")]
    Barbara,

    #[serde(rename = "Bennett")]
    Bennett,

    #[serde(rename = "Chongyun")]
    Chongyun,

    #[serde(rename = "Diona")]
    Diona,

    #[serde(rename = "Ningguang")]
    Ningguang,

    #[serde(rename = "Noel")]
    Noel,

    #[serde(rename = "NPC_CHILD")]
    NpcChild,

    #[serde(rename = "NPC_ELDER")]
    NpcElder,

    #[serde(rename = "NPC_FEMALE")]
    NpcFemale,

    #[serde(rename = "NPC_MALE")]
    NpcMale,

    #[serde(rename = "NPC_MUSCLEMAN")]
    NpcMuscleman,
}

#[derive(Serialize, Deserialize)]
pub enum Fjmjmigmjcl {
    #[serde(rename = "NPC_SPECIAL_ARANARA")]
    NpcSpecialAranara,
}

#[derive(Serialize, Deserialize)]
pub enum JsonName {
    #[serde(rename = "CanMove01")]
    CanMove01,

    #[serde(rename = "ConfigNpc_AbyssWater")]
    ConfigNpcAbyssWater,

    #[serde(rename = "ConfigNpc_AbyssWater_10201")]
    ConfigNpcAbyssWater10201,

    #[serde(rename = "ConfigNpc_AbyssWater_465")]
    ConfigNpcAbyssWater465,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove")]
    ConfigNpcCommonAnimatorMove,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Child")]
    ConfigNpcCommonAnimatorMoveChild,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Elder")]
    ConfigNpcCommonAnimatorMoveElder,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Female")]
    ConfigNpcCommonAnimatorMoveFemale,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Female_large")]
    ConfigNpcCommonAnimatorMoveFemaleLarge,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Female_LargeRadius")]
    ConfigNpcCommonAnimatorMoveFemaleLargeRadius,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Female_noheadcontrol")]
    ConfigNpcCommonAnimatorMoveFemaleNoheadcontrol,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Female_Sit")]
    ConfigNpcCommonAnimatorMoveFemaleSit,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Male")]
    ConfigNpcCommonAnimatorMoveMale,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Male_GuardRadius")]
    ConfigNpcCommonAnimatorMoveMaleGuardRadius,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Male_LargeRadius")]
    ConfigNpcCommonAnimatorMoveMaleLargeRadius,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Male_noheadcontrol_grassdis")]
    ConfigNpcCommonAnimatorMoveMaleNoheadcontrolGrassdis,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Male_Sit")]
    ConfigNpcCommonAnimatorMoveMaleSit,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_MuscleMan")]
    ConfigNpcCommonAnimatorMoveMuscleMan,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_SmallRadius")]
    ConfigNpcCommonAnimatorMoveSmallRadius,

    #[serde(rename = "ConfigNpc_Common_AnimatorMove_Xiaoming")]
    ConfigNpcCommonAnimatorMoveXiaoming,

    #[serde(rename = "ConfigNpc_Deer")]
    ConfigNpcDeer,

    #[serde(rename = "ConfigNpc_GrassSlime_10502")]
    ConfigNpcGrassSlime10502,

    #[serde(rename = "ConfigNpc_LupiBoreas_AnimatorMove")]
    ConfigNpcLupiBoreasAnimatorMove,

    #[serde(rename = "ConfigNpc_Paimon")]
    ConfigNpcPaimon,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove")]
    ConfigNpcSensingAnimatorMove,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_Adv")]
    ConfigNpcSensingAnimatorMoveAdv,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_FindCat")]
    ConfigNpcSensingAnimatorMoveFindCat,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_FollowNPC")]
    ConfigNpcSensingAnimatorMoveFollowNpc,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_Liyue")]
    ConfigNpcSensingAnimatorMoveLiyue,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_LiyueLYG")]
    ConfigNpcSensingAnimatorMoveLiyueLyg,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_SneakHideoNPC")]
    ConfigNpcSensingAnimatorMoveSneakHideoNpc,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_SneakRunNPC")]
    ConfigNpcSensingAnimatorMoveSneakRunNpc,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_SneakTodorokuNPC")]
    ConfigNpcSensingAnimatorMoveSneakTodorokuNpc,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_SneakTokudaNPC")]
    ConfigNpcSensingAnimatorMoveSneakTokudaNpc,

    #[serde(rename = "ConfigNpc_Sensing_AnimatorMove_ThrowBeanNPC")]
    ConfigNpcSensingAnimatorMoveThrowBeanNpc,

    #[serde(rename = "ConfigNpc_SneakDoor_Test")]
    ConfigNpcSneakDoorTest,

    #[serde(rename = "")]
    Empty,

    #[serde(rename = "ReadableNPC")]
    ReadableNpc,

    #[serde(rename = "ReadableNPC_Intee_StoneBoard")]
    ReadableNpcInteeStoneBoard,

    #[serde(rename = "ReadableNPC_Low")]
    ReadableNpcLow,

    #[serde(rename = "ReadableNPC_Near")]
    ReadableNpcNear,

    #[serde(rename = "StandOnly")]
    StandOnly,
}

#[derive(Serialize, Deserialize)]
pub enum LuaDataPath {
    #[serde(rename = "Actor/Npc/FindCat")]
    ActorNpcFindCat,

    #[serde(rename = "Actor/Npc/HideoSneakNPC")]
    ActorNpcHideoSneakNpc,

    #[serde(rename = "Actor/Npc/JohnDoe01")]
    ActorNpcJohnDoe01,

    #[serde(rename = "Actor/Npc/JohnDoe02")]
    ActorNpcJohnDoe02,

    #[serde(rename = "Actor/Npc/JohnDoe03")]
    ActorNpcJohnDoe03,

    #[serde(rename = "Actor/Npc/JohnDoe04")]
    ActorNpcJohnDoe04,

    #[serde(rename = "Actor/Npc/JohnDoe05")]
    ActorNpcJohnDoe05,

    #[serde(rename = "Actor/Npc/JohnDoe06")]
    ActorNpcJohnDoe06,

    #[serde(rename = "Actor/Npc/JohnDoe07")]
    ActorNpcJohnDoe07,

    #[serde(rename = "Actor/Npc/JohnDoe08")]
    ActorNpcJohnDoe08,

    #[serde(rename = "Actor/Npc/MengdeSample01")]
    ActorNpcMengdeSample01,

    #[serde(rename = "Actor/Npc/MengdeSample02")]
    ActorNpcMengdeSample02,

    #[serde(rename = "Actor/Npc/MengdeSample03")]
    ActorNpcMengdeSample03,

    #[serde(rename = "Actor/Npc/MengdeSample04")]
    ActorNpcMengdeSample04,

    #[serde(rename = "Actor/Npc/MengdeSample05")]
    ActorNpcMengdeSample05,

    #[serde(rename = "Actor/Npc/NpcEnkanomiya")]
    ActorNpcNpcEnkanomiya,

    #[serde(rename = "Actor/Npc/NpcFSMBehaviour")]
    ActorNpcNpcFsmBehaviour,

    #[serde(rename = "Actor/Npc/SneakAIRun")]
    ActorNpcSneakAiRun,

    #[serde(rename = "Actor/Npc/Soilder01")]
    ActorNpcSoilder01,

    #[serde(rename = "Actor/Npc/Soilder02")]
    ActorNpcSoilder02,

    #[serde(rename = "Actor/Npc/StoreKeeper01")]
    ActorNpcStoreKeeper01,

    #[serde(rename = "Actor/Npc/StoreKeeper02")]
    ActorNpcStoreKeeper02,

    #[serde(rename = "Actor/Npc/StoreKeeper03")]
    ActorNpcStoreKeeper03,

    #[serde(rename = "Actor/Npc/StoreKeeper04")]
    ActorNpcStoreKeeper04,

    #[serde(rename = "Actor/Npc/TempDailyNPC")]
    ActorNpcTempDailyNpc,

    #[serde(rename = "Actor/Npc/TempNPC")]
    ActorNpcTempNpc,

    #[serde(rename = "Actor/Npc/Test9001")]
    ActorNpcTest9001,

    #[serde(rename = "Actor/Npc/TodorokuSneakNPC")]
    ActorNpcTodorokuSneakNpc,

    #[serde(rename = "Actor/Npc/TokudaSneakNPC")]
    ActorNpcTokudaSneakNpc,

    #[serde(rename = "Actor/Npc/YLLQZYSneakAI")]
    ActorNpcYllqzySneakAi,

    #[serde(rename = "Actor/Npc/YunjinCoopSneakAI1")]
    ActorNpcYunjinCoopSneakAi1,

    #[serde(rename = "Actor/Npc/YunjinCoopSneakAI2")]
    ActorNpcYunjinCoopSneakAi2,

    #[serde(rename = "Actor/Quest/Q20101/GuardNPC")]
    ActorQuestQ20101GuardNpc,

    #[serde(rename = "Actor/Quest/Q301/Ambor301")]
    ActorQuestQ301Ambor301,

    #[serde(rename = "Actor/Quest/Q301/Gaia301")]
    ActorQuestQ301Gaia301,

    #[serde(rename = "Actor/Quest/Q301/Robam301")]
    ActorQuestQ301Robam301,

    #[serde(rename = "Actor/Quest/Q301/Wendy301")]
    ActorQuestQ301Wendy301,

    #[serde(rename = "Actor/Quest/Q301/WendyAudience")]
    ActorQuestQ301WendyAudience,

    #[serde(rename = "Actor/Quest/Q352/Paimon")]
    ActorQuestQ352Paimon,

    #[serde(rename = "Actor/Quest/Q376/Diluc")]
    ActorQuestQ376Diluc,

    #[serde(rename = "Actor/Quest/Q376/WendyNew")]
    ActorQuestQ376WendyNew,

    #[serde(rename = "Actor/Quest/Q411/Lisa")]
    ActorQuestQ411Lisa,

    #[serde(rename = "Actor/Quest/Q411/Qin")]
    ActorQuestQ411Qin,

    #[serde(rename = "Actor/Quest/Q413/Barbara")]
    ActorQuestQ413Barbara,

    #[serde(rename = "")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum ScriptDataPath {
    #[serde(rename = "Data/ScriptData/PropObject/SneakAI")]
    DataScriptDataPropObjectSneakAi,

    #[serde(rename = "Data/ScriptData/PropObject/SneakDoor")]
    DataScriptDataPropObjectSneakDoor,

    #[serde(rename = "Data/ScriptData/PropObject/SP_010")]
    DataScriptDataPropObjectSp010,

    #[serde(rename = "Data/ScriptData/PropObject/SP_011")]
    DataScriptDataPropObjectSp011,

    #[serde(rename = "Data/ScriptData/PropObject/SP_012")]
    DataScriptDataPropObjectSp012,

    #[serde(rename = "Data/ScriptData/PropObject/SP_013")]
    DataScriptDataPropObjectSp013,

    #[serde(rename = "Data/ScriptData/PropObject/SP_014")]
    DataScriptDataPropObjectSp014,

    #[serde(rename = "Data/ScriptData/PropObject/SP_015")]
    DataScriptDataPropObjectSp015,

    #[serde(rename = "Data/ScriptData/PropObject/SP_016")]
    DataScriptDataPropObjectSp016,

    #[serde(rename = "Data/ScriptData/PropObject/SP_017")]
    DataScriptDataPropObjectSp017,

    #[serde(rename = "Data/ScriptData/PropObject/SP_018")]
    DataScriptDataPropObjectSp018,

    #[serde(rename = "Data/ScriptData/PropObject/SP_019")]
    DataScriptDataPropObjectSp019,

    #[serde(rename = "Data/ScriptData/PropObject/SP_020")]
    DataScriptDataPropObjectSp020,

    #[serde(rename = "Data/ScriptData/PropObject/SP_021")]
    DataScriptDataPropObjectSp021,

    #[serde(rename = "Data/ScriptData/PropObject/SP_022")]
    DataScriptDataPropObjectSp022,

    #[serde(rename = "Data/ScriptData/PropObject/SP_045")]
    DataScriptDataPropObjectSp045,

    #[serde(rename = "Data/ScriptData/PropObject/SP_Invisible_Queen")]
    DataScriptDataPropObjectSpInvisibleQueen,
}

#[derive(Serialize, Deserialize)]
pub enum TemplateEmotionPath {
    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Barbara/")]
    CsEmoCoopCommonCsEmoCoopAvatarBarbara,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Beidou/")]
    CsEmoCoopCommonCsEmoCoopAvatarBeidou,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Bennett/")]
    CsEmoCoopCommonCsEmoCoopAvatarBennett,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Chongyun/")]
    CsEmoCoopCommonCsEmoCoopAvatarChongyun,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Diona/")]
    CsEmoCoopCommonCsEmoCoopAvatarDiona,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Gorou/")]
    CsEmoCoopCommonCsEmoCoopAvatarGorou,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Heizo/")]
    CsEmoCoopCommonCsEmoCoopAvatarHeizo,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Ningguang/")]
    CsEmoCoopCommonCsEmoCoopAvatarNingguang,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Noel/")]
    CsEmoCoopCommonCsEmoCoopAvatarNoel,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Sayu/")]
    CsEmoCoopCommonCsEmoCoopAvatarSayu,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Shinobu/")]
    CsEmoCoopCommonCsEmoCoopAvatarShinobu,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Tohma/")]
    CsEmoCoopCommonCsEmoCoopAvatarTohma,

    #[serde(rename = "Cs_Emo_Coop_Common/Cs_Emo_Coop_Avatar/Yunjin/")]
    CsEmoCoopCommonCsEmoCoopAvatarYunjin,

    #[serde(rename = "")]
    Empty,
}
