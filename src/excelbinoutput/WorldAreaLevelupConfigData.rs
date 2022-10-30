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

pub type WorldAreaLevelupConfigData = Vec<WorldAreaLevelupConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WorldAreaLevelupConfigDatum {
    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "areaId")]
    pub area_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "consumeItem")]
    pub consume_item: ConsumeItem,

    #[serde(rename = "actionVec")]
    pub action_vec: Vec<ActionVec>,
}

#[derive(Serialize, Deserialize)]
pub struct ActionVec {
    #[serde(rename = "param1Vec")]
    pub param1_vec: Vec<i64>,

    #[serde(rename = "param2Vec")]
    pub param2_vec: Vec<i64>,

    #[serde(rename = "type")]
    pub action_vec_type: Option<Type>,
}

#[derive(Serialize, Deserialize)]
pub struct ConsumeItem {
    #[serde(rename = "itemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "itemNum")]
    pub item_num: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "WORLD_AREA_ACTION_ACTIVATE_ITEM")]
    WorldAreaActionActivateItem,

    #[serde(rename = "WORLD_AREA_ACTION_IMPROVE_STAMINA")]
    WorldAreaActionImproveStamina,

    #[serde(rename = "WORLD_AREA_ACTION_UNLOCK_FORCE")]
    WorldAreaActionUnlockForce,
}
