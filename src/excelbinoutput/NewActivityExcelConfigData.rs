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

pub type NewActivityExcelConfigData = Vec<NewActivityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivityExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "activityType")]
    pub activity_type: String,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "activitySceneTag")]
    pub activity_scene_tag: ActivitySceneTag,

    #[serde(rename = "condGroupId")]
    pub cond_group_id: Vec<i64>,

    #[serde(rename = "watcherId")]
    pub watcher_id: Vec<i64>,

    #[serde(rename = "isLoadTerrain")]
    pub is_load_terrain: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum ActivitySceneTag {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Hdj")]
    Hdj,

    #[serde(rename = "Irodori")]
    Irodori,

    #[serde(rename = "Vintage")]
    Vintage,
}
