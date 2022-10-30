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

pub type HomeWorldFarmFieldExcelConfigData = Vec<HomeWorldFarmFieldExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldFarmFieldExcelConfigDatum {
    #[serde(rename = "GLHIOPINDIG")]
    pub glhiopindig: i64,

    #[serde(rename = "GEPIPJCAJDL")]
    pub gepipjcajdl: String,

    #[serde(rename = "LAGFJMFJIGB")]
    pub lagfjmfjigb: i64,

    #[serde(rename = "OMDABLPEFLM")]
    pub omdablpeflm: i64,

    #[serde(rename = "LFNDIDLHJHE")]
    pub lfndidlhjhe: Vec<i64>,
}
