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

pub type ActivityExcelConfigData = Vec<ActivityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityExcelConfigDatum {
    #[serde(rename = "ActivityId")]
    pub activity_id: i64,

    #[serde(rename = "Type")]
    pub activity_excel_config_datum_type: String,

    #[serde(rename = "DestroyItem")]
    pub destroy_item: Vec<DestroyItem>,

    #[serde(rename = "NameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "ActivitySceneTag")]
    pub activity_scene_tag: String,

    #[serde(rename = "IsLoadTerrain")]
    pub is_load_terrain: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct DestroyItem {
    #[serde(rename = "ItemId")]
    pub item_id: Vec<i64>,
}
