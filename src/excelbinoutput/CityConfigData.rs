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

pub type CityConfigData = Vec<CityConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CityConfigDatum {
    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "areaIdVec")]
    pub area_id_vec: Vec<i64>,

    #[serde(rename = "cityNameTextMapHash")]
    pub city_name_text_map_hash: i64,

    #[serde(rename = "mapPosX")]
    pub map_pos_x: i64,

    #[serde(rename = "mapPosY")]
    pub map_pos_y: i64,

    #[serde(rename = "zoomForExploration")]
    pub zoom_for_exploration: f64,

    #[serde(rename = "adventurePointId")]
    pub adventure_point_id: i64,

    #[serde(rename = "ExpeditionMap")]
    pub expedition_map: String,

    #[serde(rename = "ExpeditionWaterMark")]
    pub expedition_water_mark: String,

    #[serde(rename = "openState")]
    pub open_state: String,

    #[serde(rename = "cityGoddnessNameTextMapHash")]
    pub city_goddness_name_text_map_hash: i64,

    #[serde(rename = "cityGoddnessDescTextMapHash")]
    pub city_goddness_desc_text_map_hash: i64,
}
