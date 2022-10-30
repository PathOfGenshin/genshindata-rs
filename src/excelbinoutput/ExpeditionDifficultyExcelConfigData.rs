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

pub type ExpeditionDifficultyExcelConfigData = Vec<ExpeditionDifficultyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ExpeditionDifficultyExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "needHours")]
    pub need_hours: i64,

    #[serde(rename = "minPlayer")]
    pub min_player: i64,

    #[serde(rename = "maxPlayer")]
    pub max_player: i64,

    #[serde(rename = "minRefreshCount")]
    pub min_refresh_count: i64,

    #[serde(rename = "maxRefreshCount")]
    pub max_refresh_count: i64,

    #[serde(rename = "coef")]
    pub coef: f64,
}
