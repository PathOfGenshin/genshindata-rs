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

pub type InstableSprayLevelExcelConfigData = Vec<InstableSprayLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InstableSprayLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "BDEEPDFHPIL")]
    pub bdeepdfhpil: Vec<i64>,

    #[serde(rename = "KJLBELILEKP")]
    pub kjlbelilekp: Vec<i64>,

    #[serde(rename = "ANKCGJPBOKF")]
    pub ankcgjpbokf: Vec<i64>,

    #[serde(rename = "JCINIIGJFLI")]
    pub jciniigjfli: String,
}
