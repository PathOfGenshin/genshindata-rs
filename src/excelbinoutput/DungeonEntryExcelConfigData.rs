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

pub type DungeonEntryExcelConfigData = Vec<DungeonEntryExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DungeonEntryExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "dungeonEntryId")]
    pub dungeon_entry_id: i64,

    #[serde(rename = "type")]
    pub dungeon_entry_excel_config_datum_type: String,

    #[serde(rename = "isShowInAdvHandbook")]
    pub is_show_in_adv_handbook: Option<bool>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "cooldownTipsDungeonId")]
    pub cooldown_tips_dungeon_id: Vec<i64>,

    #[serde(rename = "condComb")]
    pub cond_comb: Option<CondComb>,

    #[serde(rename = "satisfiedCond")]
    pub satisfied_cond: Vec<SatisfiedCond>,

    #[serde(rename = "picPath")]
    pub pic_path: String,

    #[serde(rename = "systemOpenUiId")]
    pub system_open_ui_id: Option<i64>,

    #[serde(rename = "rewardDataId")]
    pub reward_data_id: Option<i64>,

    #[serde(rename = "descriptionCycleRewardList")]
    pub description_cycle_reward_list: Vec<Vec<i64>>,

    #[serde(rename = "isDailyRefresh")]
    pub is_daily_refresh: Option<bool>,

    #[serde(rename = "isDefaultOpen")]
    pub is_default_open: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct SatisfiedCond {
    #[serde(rename = "type")]
    pub satisfied_cond_type: Option<Type>,

    #[serde(rename = "param1")]
    pub param1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum CondComb {
    #[serde(rename = "LOGIC_OR")]
    LogicOr,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "DUNGEON_ENTRY_CONDITION_LEVEL")]
    DungeonEntryConditionLevel,

    #[serde(rename = "DUNGEON_ENTRY_CONDITION_QUEST")]
    DungeonEntryConditionQuest,
}
