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

pub type BartenderLevelExcelConfigData = Vec<BartenderLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BartenderLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "HAFOEGJLOAK")]
    pub hafoegjloak: i64,

    #[serde(rename = "LKMOEICAHLC")]
    pub lkmoeicahlc: i64,

    #[serde(rename = "MKCFNENAGLG")]
    pub mkcfnenaglg: Vec<i64>,

    #[serde(rename = "KMNPKENGINH")]
    pub kmnpkenginh: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "FGLFOAPKJPK")]
    pub fglfoapkjpk: i64,
}
