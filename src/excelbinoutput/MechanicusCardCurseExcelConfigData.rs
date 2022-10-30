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

pub type MechanicusCardCurseExcelConfigData = Vec<MechanicusCardCurseExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MechanicusCardCurseExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "effectID")]
    pub effect_id: i64,

    #[serde(rename = "lastRound")]
    pub last_round: Option<i64>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParamList")]
    pub desc_param_list: Vec<String>,
}
