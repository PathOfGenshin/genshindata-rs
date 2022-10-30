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

pub type InvestigationMonsterConfigData = Vec<InvestigationMonsterConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InvestigationMonsterConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "monsterIdList")]
    pub monster_id_list: Vec<i64>,

    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,

    #[serde(rename = "NIBFGMJAJEG")]
    pub nibfgmjajeg: Vec<i64>,

    #[serde(rename = "GDNHNDNEEKH")]
    pub gdnhndneekh: Vec<i64>,

    #[serde(rename = "DKNOHFKPCOE")]
    pub dknohfkpcoe: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "mapMarkCreateType")]
    pub map_mark_create_type: Option<MapMarkCreateType>,

    #[serde(rename = "mapMarkCreateCondition")]
    pub map_mark_create_condition: MapMarkCreateCondition,

    #[serde(rename = "monsterCategory")]
    pub monster_category: MonsterCategory,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "lockDescTextMapHash")]
    pub lock_desc_text_map_hash: i64,

    #[serde(rename = "occupiedQuestIdList")]
    pub occupied_quest_id_list: Vec<i64>,

    #[serde(rename = "unlockParentQuestId")]
    pub unlock_parent_quest_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct MapMarkCreateCondition {
    #[serde(rename = "conditionType")]
    pub condition_type: Option<ConditionType>,

    #[serde(rename = "conditionParam")]
    pub condition_param: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum ConditionType {
    #[serde(rename = "PlayerLevelGE")]
    PlayerLevelGe,
}

#[derive(Serialize, Deserialize)]
pub enum MapMarkCreateType {
    #[serde(rename = "ExtraConditions")]
    ExtraConditions,

    #[serde(rename = "NerverCreate")]
    NerverCreate,
}

#[derive(Serialize, Deserialize)]
pub enum MonsterCategory {
    #[serde(rename = "Boss")]
    Boss,

    #[serde(rename = "Common")]
    Common,

    #[serde(rename = "Elite")]
    Elite,
}
