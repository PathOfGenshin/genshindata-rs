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

pub type IrodoriChessDisorderExcelConfigData = Vec<IrodoriChessDisorderExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct IrodoriChessDisorderExcelConfigDatum {
    #[serde(rename = "GJEFDNBEJGI")]
    pub gjefdnbejgi: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}
