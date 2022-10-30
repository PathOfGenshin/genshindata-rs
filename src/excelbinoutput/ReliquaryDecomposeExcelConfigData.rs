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

pub type ReliquaryDecomposeExcelConfigData = Vec<ReliquaryDecomposeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReliquaryDecomposeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "HBIELGIJFCH")]
    pub hbielgijfch: i64,

    #[serde(rename = "CMDMGCPGOHO")]
    pub cmdmgcpgoho: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "FFBICNMGGNJ")]
    pub ffbicnmggnj: i64,

    #[serde(rename = "effectDescTextMapHash")]
    pub effect_desc_text_map_hash: i64,
}
