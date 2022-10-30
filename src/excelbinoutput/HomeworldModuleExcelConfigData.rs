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

pub type HomeworldModuleExcelConfigData = Vec<HomeworldModuleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeworldModuleExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "EJAPABPJFBA")]
    pub ejapabpjfba: i64,

    #[serde(rename = "worldSceneId")]
    pub world_scene_id: i64,

    #[serde(rename = "defaultRoomSceneId")]
    pub default_room_scene_id: i64,

    #[serde(rename = "optionalRoomSceneIdVec")]
    pub optional_room_scene_id_vec: Vec<i64>,

    #[serde(rename = "moduleNameTextMapHash")]
    pub module_name_text_map_hash: i64,

    #[serde(rename = "moduleDescTextMapHash")]
    pub module_desc_text_map_hash: i64,

    #[serde(rename = "region")]
    pub region: Vec<String>,

    #[serde(rename = "regionPointPos")]
    pub region_point_pos: Vec<String>,

    #[serde(rename = "smallImageAddr")]
    pub small_image_addr: String,

    #[serde(rename = "bigImageAddr")]
    pub big_image_addr: String,

    #[serde(rename = "isFree")]
    pub is_free: Option<bool>,
}
