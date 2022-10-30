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

pub type HomeWorldFurnitureTypeExcelConfigData = Vec<HomeWorldFurnitureTypeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldFurnitureTypeExcelConfigDatum {
    #[serde(rename = "typeID")]
    pub type_id: i64,

    #[serde(rename = "CCEOKJDDNPP")]
    pub cceokjddnpp: i64,

    #[serde(rename = "typeNameTextMapHash")]
    pub type_name_text_map_hash: i64,

    #[serde(rename = "typeName2TextMapHash")]
    pub type_name2_text_map_hash: i64,

    #[serde(rename = "tabIcon")]
    pub tab_icon: String,

    #[serde(rename = "isShowInBag")]
    pub is_show_in_bag: Option<bool>,

    #[serde(rename = "sceneType")]
    pub scene_type: Option<SceneType>,

    #[serde(rename = "CDOPFMAHELD")]
    pub cdopfmaheld: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum SceneType {
    #[serde(rename = "Exterior")]
    Exterior,
}
