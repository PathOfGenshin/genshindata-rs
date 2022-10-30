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

pub type TreasureSeelieRegionExcelConfigData = Vec<TreasureSeelieRegionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TreasureSeelieRegionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "HNMCJJJHLKL")]
    pub hnmcjjjhlkl: i64,

    #[serde(rename = "IMFOJOOIAAM")]
    pub imfojooiaam: Vec<i64>,

    #[serde(rename = "GBJLPOMBEGG")]
    pub gbjlpombegg: i64,
}
