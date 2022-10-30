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

pub type EnvAnimalGatherExcelConfigData = Vec<EnvAnimalGatherExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EnvAnimalGatherExcelConfigDatum {
    #[serde(rename = "animalId")]
    pub animal_id: i64,

    #[serde(rename = "areaId")]
    pub area_id: i64,

    #[serde(rename = "entityType")]
    pub entity_type: EntityType,

    #[serde(rename = "gatherItemId")]
    pub gather_item_id: Vec<GatherItemId>,

    #[serde(rename = "escapeRadius")]
    pub escape_radius: Option<i64>,

    #[serde(rename = "escapeTime")]
    pub escape_time: Option<i64>,

    #[serde(rename = "aliveTime")]
    pub alive_time: Option<i64>,

    #[serde(rename = "excludeWeathers")]
    pub exclude_weathers: ExcludeWeathers,
}

#[derive(Serialize, Deserialize)]
pub struct GatherItemId {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "Gadget")]
    Gadget,

    #[serde(rename = "Monster")]
    Monster,
}

#[derive(Serialize, Deserialize)]
pub enum ExcludeWeathers {
    #[serde(rename = "雨,雷雨,雪")]
    Empty,

    #[serde(rename = "")]
    ExcludeWeathers,
}
