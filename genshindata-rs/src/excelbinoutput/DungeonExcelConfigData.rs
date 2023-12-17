/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type DungeonExcelConfigData = Vec<DungeonExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DungeonExcelConfigDatum {
    pub id: i64,
    pub name_text_map_hash: i64,
    pub display_name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    #[serde(rename = "type")]
    pub dungeon_excel_config_datum_type: String,
    pub scene_id: i64,
    pub involve_type: InvolveType,
    pub show_level: Option<i64>,
    pub limit_level: Option<i64>,
    pub level_revise: Option<i64>,
    pub pass_cond: Option<i64>,
    pub revive_max_count: Option<i64>,
    pub day_enter_count: Option<i64>,
    pub enter_cost_items: Vec<EnterCostItem>,
    #[serde(rename = "passRewardPreviewID")]
    pub pass_reward_preview_id: Option<i64>,
    pub settle_countdown_time: Option<i64>,
    pub fail_settle_countdown_time: Option<i64>,
    pub quit_settle_countdown_time: Option<i64>,
    pub settle_shows: Vec<SettleShow>,
    pub forbidden_restart: Option<bool>,
    #[serde(rename = "settleUIType")]
    pub settle_ui_type: Option<SettleUiType>,
    pub recommend_element_types: Vec<RecommendElementType>,
    pub recommend_tips: Vec<String>,
    pub level_config_map: HashMap<String, i64>,
    pub preview_monster_list: Vec<i64>,
    pub gear_desc_text_map_hash: i64,
    #[serde(rename = "cityID")]
    pub city_id: i64,
    pub entry_pic_path: String,
    pub state_type: StateType,
    pub factor_pic: FactorPic,
    pub factor_icon: FactorIcon,
    #[serde(rename = "JDBDFNDPIIB")]
    pub jdbdfndpiib: i64,
    #[serde(rename = "MMDLLFFDGOD")]
    pub mmdllffdgod: i64,
    pub avatar_limit_type: Option<i64>,
    pub is_dynamic_level: Option<bool>,
    #[serde(rename = "DKJCCPMNIIH")]
    pub dkjccpmniih: Option<bool>,
    #[serde(rename = "PNJNOFEKHFI")]
    pub pnjnofekhfi: Option<Pnjnofekhfi>,
    pub sub_type: Option<SubType>,
    pub serial_id: Option<i64>,
    #[serde(rename = "OMCIDDLJAFJ")]
    pub omciddljafj: Option<bool>,
    #[serde(rename = "FKFBJPIJGBO")]
    pub fkfbjpijgbo: Option<bool>,
    pub enable_quest_guide: Option<bool>,
    #[serde(rename = "firstPassRewardPreviewID")]
    pub first_pass_reward_preview_id: Option<i64>,
    #[serde(rename = "MDPIGALKLNJ")]
    pub mdpigalklnj: Option<bool>,
    pub pass_jump_dungeon: Option<i64>,
    #[serde(rename = "APELADOEGAF")]
    pub apeladoegaf: Option<String>,
    pub dont_show_push_tips: Option<bool>,
    pub play_type: Option<PlayType>,
    pub event_interval: Option<i64>,
    #[serde(rename = "BNGIDBJGCNP")]
    pub bngidbjgcnp: Option<i64>,
    pub revive_interval_time: Option<i64>,
    #[serde(rename = "PLAMEMGOOKN")]
    pub plamemgookn: Option<bool>,
    #[serde(rename = "statueCostID")]
    pub statue_cost_id: Option<i64>,
    pub statue_cost_count: Option<i64>,
    pub statue_drop: Option<i64>,
    #[serde(rename = "PFHFJHBKHAJ")]
    pub pfhfjhbkhaj: Option<bool>,
    #[serde(rename = "PLIFPPIFOCM")]
    pub plifppifocm: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterCostItem {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactorIcon {
    #[serde(rename = "ART/UI/Atlas/DungeonTypeIcon/UI_Icon_TeamChain")]
    ArtUiAtlasDungeonTypeIconUiIconTeamChain,
    #[serde(rename = "ART/UI/Atlas/ElementIcons/UI_Icon_Climate_LaSignora")]
    ArtUiAtlasElementIconsUiIconClimateLaSignora,
    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactorPic {
    #[serde(rename = "ART/UI/Atlas/ClimatePic/UI_Img_Climate_LaSignora")]
    ArtUiAtlasClimatePicUiImgClimateLaSignora,
    #[serde(rename = "ART/UI/Atlas/DungeonPic/UI_DungeonPic_InspirationSpurt")]
    ArtUiAtlasDungeonPicUiDungeonPicInspirationSpurt,
    #[serde(rename = "ART/UI/Atlas/DungeonPic/UI_DungeonPic_MultiCharacter")]
    ArtUiAtlasDungeonPicUiDungeonPicMultiCharacter,
    #[serde(rename = "ART/UI/Atlas/DungeonPic/UI_DungeonPic_TeamChain")]
    ArtUiAtlasDungeonPicUiDungeonPicTeamChain,
    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvolveType {
    #[serde(rename = "INVOLVE_ONLY_SINGLE")]
    InvolveOnlySingle,
    #[serde(rename = "INVOLVE_SINGLE_MULTIPLE")]
    InvolveSingleMultiple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayType {
    #[serde(rename = "DUNGEON_PLAY_TYPE_FOGGY_MAZE")]
    DungeonPlayTypeFoggyMaze,
    #[serde(rename = "DUNGEON_PLAY_TYPE_MIST_TRIAL")]
    DungeonPlayTypeMistTrial,
    #[serde(rename = "DUNGEON_PLAY_TYPE_TRIAL_AVATAR")]
    DungeonPlayTypeTrialAvatar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Pnjnofekhfi {
    #[serde(rename = "DUNGEON_DIFFICULTY_TYPE_REDUCE_1")]
    DungeonDifficultyTypeReduce1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendElementType {
    Electric,
    Fire,
    Grass,
    Ice,
    None,
    Rock,
    Water,
    Wind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettleUiType {
    #[serde(rename = "SETTLE_UI_NEVER_SHOW")]
    SettleUiNeverShow,
    #[serde(rename = "SETTLE_UI_ON_FAIL")]
    SettleUiOnFail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StateType {
    #[serde(rename = "DUNGEON_STATE_RELEASE")]
    DungeonStateRelease,
    #[serde(rename = "DUNGEON_STATE_TEST")]
    DungeonStateTest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
