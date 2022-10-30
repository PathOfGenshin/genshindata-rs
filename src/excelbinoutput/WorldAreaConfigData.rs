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

pub type WorldAreaConfigData = Vec<WorldAreaConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WorldAreaConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "SceneID")]
    pub scene_id: i64,

    #[serde(rename = "AreaType")]
    pub area_type: AreaType,

    #[serde(rename = "AreaID1")]
    pub area_id1: i64,

    #[serde(rename = "AreaNameTextMapHash")]
    pub area_name_text_map_hash: i64,

    #[serde(rename = "towerPointId")]
    pub tower_point_id: Option<i64>,

    #[serde(rename = "elementType")]
    pub element_type: Option<ElementType>,

    #[serde(rename = "showTips")]
    pub show_tips: Option<bool>,

    #[serde(rename = "minimapScale")]
    pub minimap_scale: f64,

    #[serde(rename = "AreaID2")]
    pub area_id2: Option<i64>,

    #[serde(rename = "terrainType")]
    pub terrain_type: Option<TerrainType>,

    #[serde(rename = "AreaDefaultLock")]
    pub area_default_lock: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum AreaType {
    #[serde(rename = "LEVEL_1")]
    Level1,

    #[serde(rename = "LEVEL_2")]
    Level2,
}

#[derive(Serialize, Deserialize)]
pub enum ElementType {
    #[serde(rename = "Electric")]
    Electric,

    #[serde(rename = "Grass")]
    Grass,

    #[serde(rename = "Rock")]
    Rock,

    #[serde(rename = "Wind")]
    Wind,
}

#[derive(Serialize, Deserialize)]
pub enum TerrainType {
    #[serde(rename = "AREA_TERRAIN_CITY")]
    AreaTerrainCity,

    #[serde(rename = "AREA_TERRAIN_OUTDOOR")]
    AreaTerrainOutdoor,
}
