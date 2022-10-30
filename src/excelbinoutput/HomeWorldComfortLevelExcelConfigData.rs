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

pub type HomeWorldComfortLevelExcelConfigData = Vec<HomeWorldComfortLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldComfortLevelExcelConfigDatum {
    #[serde(rename = "levelID")]
    pub level_id: i64,

    #[serde(rename = "homeCoinProduceRate")]
    pub home_coin_produce_rate: i64,

    #[serde(rename = "companionshipExpProduceRate")]
    pub companionship_exp_produce_rate: i64,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "levelIconHashSuffix")]
    pub level_icon_hash_suffix: i64,

    #[serde(rename = "levelIconHashPre")]
    pub level_icon_hash_pre: i64,

    #[serde(rename = "comfort")]
    pub comfort: Option<i64>,
}
