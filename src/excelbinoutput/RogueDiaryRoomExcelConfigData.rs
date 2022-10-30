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

pub type RogueDiaryRoomExcelConfigData = Vec<RogueDiaryRoomExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueDiaryRoomExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "NLHFGOKJCFO")]
    pub nlhfgokjcfo: i64,

    #[serde(rename = "challengeId")]
    pub challenge_id: Option<i64>,

    #[serde(rename = "NGEHEHGLFGJ")]
    pub ngehehglfgj: Option<i64>,

    #[serde(rename = "AJFCIGLPHFF")]
    pub ajfciglphff: Option<i64>,

    #[serde(rename = "MDHHNFKDAEC")]
    pub mdhhnfkdaec: i64,

    #[serde(rename = "BFEOCAMBGMA")]
    pub bfeocambgma: Option<i64>,

    #[serde(rename = "BAMFBHIBGIO")]
    pub bamfbhibgio: Option<bool>,
}
