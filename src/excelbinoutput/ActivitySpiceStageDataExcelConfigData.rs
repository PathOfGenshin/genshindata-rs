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

pub type ActivitySpiceStageDataExcelConfigData = Vec<ActivitySpiceStageDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySpiceStageDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "BEGKAFFEDAI")]
    pub begkaffedai: i64,

    #[serde(rename = "KHMIGGLNHHG")]
    pub khmigglnhhg: i64,

    #[serde(rename = "KLMHDEIHIFM")]
    pub klmhdeihifm: Vec<i64>,

    #[serde(rename = "BMEOPCKCNDC")]
    pub bmeopckcndc: Vec<i64>,

    #[serde(rename = "JKIDOBONNPP")]
    pub jkidobonnpp: Vec<i64>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "IGAEPOBPFFK")]
    pub igaepobpffk: f64,

    #[serde(rename = "times")]
    pub times: i64,

    #[serde(rename = "OEJFJNOCHPD")]
    pub oejfjnochpd: i64,

    #[serde(rename = "LMODIMDIFAI")]
    pub lmodimdifai: Vec<i64>,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,
}
