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

pub type WorldAreaExploreEventConfigData = Vec<WorldAreaExploreEventConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WorldAreaExploreEventConfigDatum {
    #[serde(rename = "EventID")]
    pub event_id: i64,

    #[serde(rename = "SceneID")]
    pub scene_id: i64,

    #[serde(rename = "AreaID")]
    pub area_id: i64,

    #[serde(rename = "EventType")]
    pub event_type: EventType,

    #[serde(rename = "Param")]
    pub param: Vec<String>,

    #[serde(rename = "ExploreWeight")]
    pub explore_weight: i64,
}

#[derive(Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "EXPLORE_EVENT_CLEAR_GROUP_MONSTER")]
    ExploreEventClearGroupMonster,

    #[serde(rename = "EXPLORE_EVENT_ENTER_FORCE")]
    ExploreEventEnterForce,

    #[serde(rename = "EXPLORE_EVENT_ITEM_ADD")]
    ExploreEventItemAdd,

    #[serde(rename = "EXPLORE_EVENT_OPEN_CHEST")]
    ExploreEventOpenChest,

    #[serde(rename = "EXPLORE_EVENT_UNLOCK_POINT")]
    ExploreEventUnlockPoint,
}
