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

pub type ViewCodexExcelConfigData = Vec<ViewCodexExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ViewCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "image")]
    pub image: String,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "worldAreaId")]
    pub world_area_id: i64,

    #[serde(rename = "SortOrder")]
    pub sort_order: i64,

    #[serde(rename = "showOnlyUnlocked")]
    pub show_only_unlocked: Option<bool>,
}
