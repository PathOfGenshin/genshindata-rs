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

pub type LanV2FireworksStageDataExcelConfigData = Vec<LanV2FireworksStageDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LanV2FireworksStageDataExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "challengeIdList")]
    pub challenge_id_list: Vec<i64>,

    #[serde(rename = "tabNameTextMapHash")]
    pub tab_name_text_map_hash: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,

    #[serde(rename = "EANDOKCPMGN")]
    pub eandokcpmgn: Vec<i64>,

    #[serde(rename = "EHBGHEDHMII")]
    pub ehbghedhmii: i64,
}
