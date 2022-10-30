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

pub type FleurFairDungeonStatExcelConfigData = Vec<FleurFairDungeonStatExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FleurFairDungeonStatExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: Option<i64>,

    #[serde(rename = "statType")]
    pub stat_type: StatType,

    #[serde(rename = "statParamList")]
    pub stat_param_list: Vec<String>,

    #[serde(rename = "orderingType")]
    pub ordering_type: OrderingType,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "statMethod")]
    pub stat_method: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum OrderingType {
    #[serde(rename = "Equal")]
    Equal,

    #[serde(rename = "Greater")]
    Greater,

    #[serde(rename = "LessOrEqual")]
    LessOrEqual,
}

#[derive(Serialize, Deserialize)]
pub enum StatType {
    #[serde(rename = "FLEUR_FAIR_DUNGEON_STAT_GROUP_VARIABLE")]
    FleurFairDungeonStatGroupVariable,

    #[serde(rename = "FLEUR_FAIR_DUNGEON_STAT_MONSTER_HURT")]
    FleurFairDungeonStatMonsterHurt,
}
