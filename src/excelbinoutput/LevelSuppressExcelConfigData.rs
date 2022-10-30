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

pub type LevelSuppressExcelConfigData = Vec<LevelSuppressExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LevelSuppressExcelConfigDatum {
    #[serde(rename = "levelSuppressDamageCo")]
    pub level_suppress_damage_co: f64,

    #[serde(rename = "levelSuppressEndure")]
    pub level_suppress_endure: f64,

    #[serde(rename = "levelSuppressDisMinHorizontal")]
    pub level_suppress_dis_min_horizontal: f64,

    #[serde(rename = "levelSuppressDisMaxHorizontal")]
    pub level_suppress_dis_max_horizontal: f64,

    #[serde(rename = "levelSuppressDisMinVertical")]
    pub level_suppress_dis_min_vertical: f64,

    #[serde(rename = "levelSuppressDisMaxVertical")]
    pub level_suppress_dis_max_vertical: f64,

    #[serde(rename = "isAttackerPlayer")]
    pub is_attacker_player: Option<bool>,

    #[serde(rename = "isDefenserPlayer")]
    pub is_defenser_player: Option<bool>,

    #[serde(rename = "level")]
    pub level: Option<i64>,
}
