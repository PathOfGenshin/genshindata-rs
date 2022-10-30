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

pub type MatchExcelConfigData = Vec<MatchExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MatchExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "matchSubType")]
    pub match_sub_type: String,

    #[serde(rename = "minPlayerNum")]
    pub min_player_num: i64,

    #[serde(rename = "maxPlayerNum")]
    pub max_player_num: i64,

    #[serde(rename = "confirmTime")]
    pub confirm_time: i64,
}
