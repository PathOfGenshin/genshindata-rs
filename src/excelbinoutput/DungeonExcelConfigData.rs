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
use std::collections::HashMap;

pub type DungeonExcelConfigData = Vec<DungeonExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DungeonExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "displayNameTextMapHash")]
    pub display_name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "type")]
    pub dungeon_excel_config_datum_type: Type,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "involveType")]
    pub involve_type: InvolveType,

    #[serde(rename = "showLevel")]
    pub show_level: Option<i64>,

    #[serde(rename = "limitLevel")]
    pub limit_level: Option<i64>,

    #[serde(rename = "levelRevise")]
    pub level_revise: Option<i64>,

    #[serde(rename = "passCond")]
    pub pass_cond: Option<i64>,

    #[serde(rename = "reviveMaxCount")]
    pub revive_max_count: Option<i64>,

    #[serde(rename = "dayEnterCount")]
    pub day_enter_count: Option<i64>,

    #[serde(rename = "JNHMFILLDCE")]
    pub jnhmfilldce: Vec<Jnhmfilldce>,

    #[serde(rename = "passRewardPreviewID")]
    pub pass_reward_preview_id: Option<i64>,

    #[serde(rename = "settleCountdownTime")]
    pub settle_countdown_time: i64,

    #[serde(rename = "failSettleCountdownTime")]
    pub fail_settle_countdown_time: Option<i64>,

    #[serde(rename = "quitSettleCountdownTime")]
    pub quit_settle_countdown_time: i64,

    #[serde(rename = "settleShows")]
    pub settle_shows: Vec<SettleShow>,

    #[serde(rename = "forbiddenRestart")]
    pub forbidden_restart: Option<bool>,

    #[serde(rename = "settleUIType")]
    pub settle_ui_type: Option<SettleUiType>,

    #[serde(rename = "recommendElementTypes")]
    pub recommend_element_types: Vec<RecommendElementType>,

    #[serde(rename = "FEOFOGNECFH")]
    pub feofognecfh: Vec<String>,

    #[serde(rename = "levelConfigMap")]
    pub level_config_map: HashMap<String, i64>,

    #[serde(rename = "enterCostItems")]
    pub enter_cost_items: Vec<i64>,

    #[serde(rename = "MKCJLIGKICK")]
    pub mkcjligkick: i64,

    #[serde(rename = "cityID")]
    pub city_id: Option<i64>,

    #[serde(rename = "entryPicPath")]
    pub entry_pic_path: String,

    #[serde(rename = "stateType")]
    pub state_type: StateType,

    #[serde(rename = "HKGDKECPNEA")]
    pub hkgdkecpnea: Hkgdkecpnea,

    #[serde(rename = "PFOLCFNOIGF")]
    pub pfolcfnoigf: Pfolcfnoigf,

    #[serde(rename = "avatarLimitType")]
    pub avatar_limit_type: Option<i64>,

    #[serde(rename = "isDynamicLevel")]
    pub is_dynamic_level: Option<bool>,

    #[serde(rename = "subType")]
    pub sub_type: Option<SubType>,

    #[serde(rename = "serialId")]
    pub serial_id: Option<i64>,

    #[serde(rename = "CEJNGDKNBFP")]
    pub cejngdknbfp: Option<bool>,

    #[serde(rename = "JMLFEDHFPNL")]
    pub jmlfedhfpnl: Option<bool>,

    #[serde(rename = "firstPassRewardPreviewID")]
    pub first_pass_reward_preview_id: Option<i64>,

    #[serde(rename = "passJumpDungeon")]
    pub pass_jump_dungeon: Option<i64>,

    #[serde(rename = "dontShowPushTips")]
    pub dont_show_push_tips: Option<bool>,

    #[serde(rename = "playType")]
    pub play_type: Option<PlayType>,

    #[serde(rename = "eventInterval")]
    pub event_interval: Option<i64>,

    #[serde(rename = "reviveIntervalTime")]
    pub revive_interval_time: Option<i64>,

    #[serde(rename = "statueCostID")]
    pub statue_cost_id: Option<i64>,

    #[serde(rename = "statueCostCount")]
    pub statue_cost_count: Option<i64>,

    #[serde(rename = "statueDrop")]
    pub statue_drop: Option<i64>,

    #[serde(rename = "IHKGGJBNLIG")]
    pub ihkggjbnlig: Option<bool>,

    #[serde(rename = "CADCHBEJFLN")]
    pub cadchbejfln: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Jnhmfilldce {}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "DUNGEON_ACTIVITY")]
    DungeonActivity,

    #[serde(rename = "DUNGEON_BLITZ_RUSH")]
    DungeonBlitzRush,

    #[serde(rename = "DUNGEON_BOSS")]
    DungeonBoss,

    #[serde(rename = "DUNGEON_CHANNELLER_SLAB_LOOP")]
    DungeonChannellerSlabLoop,

    #[serde(rename = "DUNGEON_CHANNELLER_SLAB_ONE_OFF")]
    DungeonChannellerSlabOneOff,

    #[serde(rename = "DUNGEON_CHESS")]
    DungeonChess,

    #[serde(rename = "DUNGEON_CRYSTAL_LINK")]
    DungeonCrystalLink,

    #[serde(rename = "DUNGEON_DAILY_FIGHT")]
    DungeonDailyFight,

    #[serde(rename = "DUNGEON_DISCARDED")]
    DungeonDiscarded,

    #[serde(rename = "DUNGEON_DREAMLAND")]
    DungeonDreamland,

    #[serde(rename = "DUNGEON_EFFIGY")]
    DungeonEffigy,

    #[serde(rename = "DUNGEON_ELEMENT_CHALLENGE")]
    DungeonElementChallenge,

    #[serde(rename = "DUNGEON_FIGHT")]
    DungeonFight,

    #[serde(rename = "DUNGEON_FLEUR_FAIR")]
    DungeonFleurFair,

    #[serde(rename = "DUNGEON_HACHI")]
    DungeonHachi,

    #[serde(rename = "DUNGEON_INSTABLE_SPRAY")]
    DungeonInstableSpray,

    #[serde(rename = "DUNGEON_IRODORI_CHESS")]
    DungeonIrodoriChess,

    #[serde(rename = "DUNGEON_MINI_ELDRITCH")]
    DungeonMiniEldritch,

    #[serde(rename = "DUNGEON_MUQADAS_POTION")]
    DungeonMuqadasPotion,

    #[serde(rename = "DUNGEON_PLOT")]
    DungeonPlot,

    #[serde(rename = "DUNGEON_POTION")]
    DungeonPotion,

    #[serde(rename = "DUNGEON_ROGUE_DIARY")]
    DungeonRogueDiary,

    #[serde(rename = "DUNGEON_ROGUELIKE")]
    DungeonRoguelike,

    #[serde(rename = "DUNGEON_SUMMER_V2")]
    DungeonSummerV2,

    #[serde(rename = "DUNGEON_SUMO_COMBAT")]
    DungeonSumoCombat,

    #[serde(rename = "DUNGEON_THEATRE_MECHANICUS")]
    DungeonTheatreMechanicus,

    #[serde(rename = "DUNGEON_TOWER")]
    DungeonTower,

    #[serde(rename = "DUNGEON_UGC")]
    DungeonUgc,

    #[serde(rename = "DUNGEON_WIND_FIELD")]
    DungeonWindField,
}

#[derive(Serialize, Deserialize)]
pub enum Hkgdkecpnea {
    #[serde(rename = "ART/UI/Atlas/ClimatePic/UI_Img_Climate_LaSignora")]
    ArtUiAtlasClimatePicUiImgClimateLaSignora,

    #[serde(rename = "")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum InvolveType {
    #[serde(rename = "INVOLVE_ONLY_SINGLE")]
    InvolveOnlySingle,

    #[serde(rename = "INVOLVE_SINGLE_MULTIPLE")]
    InvolveSingleMultiple,
}

#[derive(Serialize, Deserialize)]
pub enum Pfolcfnoigf {
    #[serde(rename = "ART/UI/Atlas/ElementIcons/UI_Icon_Climate_LaSignora")]
    ArtUiAtlasElementIconsUiIconClimateLaSignora,

    #[serde(rename = "")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum PlayType {
    #[serde(rename = "DUNGEON_PLAY_TYPE_FOGGY_MAZE")]
    DungeonPlayTypeFoggyMaze,

    #[serde(rename = "DUNGEON_PLAY_TYPE_MIST_TRIAL")]
    DungeonPlayTypeMistTrial,

    #[serde(rename = "DUNGEON_PLAY_TYPE_TRIAL_AVATAR")]
    DungeonPlayTypeTrialAvatar,
}

#[derive(Serialize, Deserialize)]
pub enum RecommendElementType {
    #[serde(rename = "Electric")]
    Electric,

    #[serde(rename = "Fire")]
    Fire,

    #[serde(rename = "Grass")]
    Grass,

    #[serde(rename = "Ice")]
    Ice,

    #[serde(rename = "None")]
    None,

    #[serde(rename = "Rock")]
    Rock,

    #[serde(rename = "Water")]
    Water,

    #[serde(rename = "Wind")]
    Wind,
}

#[derive(Serialize, Deserialize)]
pub enum SettleShow {
    #[serde(rename = "SETTLE_SHOW_BLACKSCREEN")]
    SettleShowBlackscreen,

    #[serde(rename = "SETTLE_SHOW_KILL_MONSTER_COUNT")]
    SettleShowKillMonsterCount,

    #[serde(rename = "SETTLE_SHOW_NONE")]
    SettleShowNone,

    #[serde(rename = "SETTLE_SHOW_OPEN_CHEST_COUNT")]
    SettleShowOpenChestCount,

    #[serde(rename = "SETTLE_SHOW_TIME_COST")]
    SettleShowTimeCost,
}

#[derive(Serialize, Deserialize)]
pub enum SettleUiType {
    #[serde(rename = "SETTLE_UI_NEVER_SHOW")]
    SettleUiNeverShow,

    #[serde(rename = "SETTLE_UI_ON_FAIL")]
    SettleUiOnFail,
}

#[derive(Serialize, Deserialize)]
pub enum StateType {
    #[serde(rename = "DUNGEON_STATE_RELEASE")]
    DungeonStateRelease,

    #[serde(rename = "DUNGEON_STATE_TEST")]
    DungeonStateTest,
}

#[derive(Serialize, Deserialize)]
pub enum SubType {
    #[serde(rename = "DUNGEON_SUB_BOSS")]
    DungeonSubBoss,

    #[serde(rename = "DUNGEON_SUB_RELIQUARY")]
    DungeonSubReliquary,

    #[serde(rename = "DUNGEON_SUB_TALENT")]
    DungeonSubTalent,

    #[serde(rename = "DUNGEON_SUB_WEAPON")]
    DungeonSubWeapon,
}
