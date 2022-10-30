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

pub type MainCoopExcelConfigData = Vec<MainCoopExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MainCoopExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "mainCoopCfg")]
    pub main_coop_cfg: String,

    #[serde(rename = "mainQuestSeries")]
    pub main_quest_series: i64,
}