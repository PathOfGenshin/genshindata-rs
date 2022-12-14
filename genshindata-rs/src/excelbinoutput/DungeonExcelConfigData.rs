// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;
use std::collections::HashMap;

pub type DungeonExcelConfigData = Vec<DungeonExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
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
    pub dungeon_excel_config_datum_type: String,

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

    #[serde(rename = "FCCFDEMHGEO")]
    pub fccfdemhgeo: Vec<Fccfdemhgeo>,

    #[serde(rename = "passRewardPreviewID")]
    pub pass_reward_preview_id: Option<i64>,

    #[serde(rename = "settleCountdownTime")]
    pub settle_countdown_time: Option<i64>,

    #[serde(rename = "failSettleCountdownTime")]
    pub fail_settle_countdown_time: Option<i64>,

    #[serde(rename = "quitSettleCountdownTime")]
    pub quit_settle_countdown_time: Option<i64>,

    #[serde(rename = "settleShows")]
    pub settle_shows: Vec<SettleShow>,

    #[serde(rename = "forbiddenRestart")]
    pub forbidden_restart: Option<bool>,

    #[serde(rename = "settleUIType")]
    pub settle_ui_type: Option<SettleUiType>,

    #[serde(rename = "recommendElementTypes")]
    pub recommend_element_types: Vec<RecommendElementType>,

    #[serde(rename = "NILCBBCGJLC")]
    pub nilcbbcgjlc: Vec<String>,

    #[serde(rename = "levelConfigMap")]
    pub level_config_map: HashMap<String, i64>,

    #[serde(rename = "enterCostItems")]
    pub enter_cost_items: Vec<i64>,

    #[serde(rename = "HCMALEDDJAH")]
    pub hcmaleddjah: i64,

    #[serde(rename = "cityID")]
    pub city_id: i64,

    #[serde(rename = "entryPicPath")]
    pub entry_pic_path: String,

    #[serde(rename = "stateType")]
    pub state_type: StateType,

    #[serde(rename = "FEDJEJFCHHO")]
    pub fedjejfchho: Fedjejfchho,

    #[serde(rename = "BIMCHJDGJHH")]
    pub bimchjdgjhh: Bimchjdgjhh,

    #[serde(rename = "avatarLimitType")]
    pub avatar_limit_type: Option<i64>,

    #[serde(rename = "isDynamicLevel")]
    pub is_dynamic_level: Option<bool>,

    #[serde(rename = "subType")]
    pub sub_type: Option<SubType>,

    #[serde(rename = "serialId")]
    pub serial_id: Option<i64>,

    #[serde(rename = "LHANHABLMGM")]
    pub lhanhablmgm: Option<bool>,

    #[serde(rename = "LCNACOLGDDC")]
    pub lcnacolgddc: Option<bool>,

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

    #[serde(rename = "KILJGGKIOAE")]
    pub kiljggkioae: Option<bool>,

    #[serde(rename = "statueCostID")]
    pub statue_cost_id: Option<i64>,

    #[serde(rename = "statueCostCount")]
    pub statue_cost_count: Option<i64>,

    #[serde(rename = "statueDrop")]
    pub statue_drop: Option<i64>,

    #[serde(rename = "IEMGOIMAOCM")]
    pub iemgoimaocm: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fccfdemhgeo {
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Bimchjdgjhh {
    #[serde(rename = "ART/UI/Atlas/ElementIcons/UI_Icon_Climate_LaSignora")]
    ArtUiAtlasElementIconsUiIconClimateLaSignora,

    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Fedjejfchho {
    #[serde(rename = "ART/UI/Atlas/ClimatePic/UI_Img_Climate_LaSignora")]
    ArtUiAtlasClimatePicUiImgClimateLaSignora,

    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InvolveType {
    #[serde(rename = "INVOLVE_ONLY_SINGLE")]
    InvolveOnlySingle,

    #[serde(rename = "INVOLVE_SINGLE_MULTIPLE")]
    InvolveSingleMultiple,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayType {
    #[serde(rename = "DUNGEON_PLAY_TYPE_FOGGY_MAZE")]
    DungeonPlayTypeFoggyMaze,

    #[serde(rename = "DUNGEON_PLAY_TYPE_MIST_TRIAL")]
    DungeonPlayTypeMistTrial,

    #[serde(rename = "DUNGEON_PLAY_TYPE_TRIAL_AVATAR")]
    DungeonPlayTypeTrialAvatar,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum SettleUiType {
    #[serde(rename = "SETTLE_UI_NEVER_SHOW")]
    SettleUiNeverShow,

    #[serde(rename = "SETTLE_UI_ON_FAIL")]
    SettleUiOnFail,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StateType {
    #[serde(rename = "DUNGEON_STATE_RELEASE")]
    DungeonStateRelease,

    #[serde(rename = "DUNGEON_STATE_TEST")]
    DungeonStateTest,
}

#[derive(Debug, Serialize, Deserialize)]
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

pub fn load() -> Result<DungeonExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DungeonExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
