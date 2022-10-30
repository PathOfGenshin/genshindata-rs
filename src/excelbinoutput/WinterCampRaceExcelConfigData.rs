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

pub type WinterCampRaceExcelConfigData = Vec<WinterCampRaceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WinterCampRaceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "PGGOKANNJLJ")]
    pub pggokannjlj: i64,

    #[serde(rename = "OBPCFJMMNNA")]
    pub obpcfjmmnna: Vec<i64>,

    #[serde(rename = "JJLGHIANHCL")]
    pub jjlghianhcl: Vec<i64>,

    #[serde(rename = "JFPFDLGPAIG")]
    pub jfpfdlgpaig: i64,

    #[serde(rename = "JJNEDDLGJHH")]
    pub jjneddlgjhh: i64,

    #[serde(rename = "FJIJAJAGHGI")]
    pub fjijajaghgi: i64,

    #[serde(rename = "FCNBJIIAFKP")]
    pub fcnbjiiafkp: Vec<i64>,
}
