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

pub type BounceConjuringChapterExcelConfigData = Vec<BounceConjuringChapterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BounceConjuringChapterExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "DFBKNDGGDNG")]
    pub dfbkndggdng: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KJGDNLKGEDD")]
    pub kjgdnlkgedd: Vec<i64>,

    #[serde(rename = "OPCHHKKOOKN")]
    pub opchhkkookn: Vec<i64>,

    #[serde(rename = "AHLJPEKCMLD")]
    pub ahljpekcmld: Vec<i64>,

    #[serde(rename = "MGDMGOLBFMI")]
    pub mgdmgolbfmi: Vec<Option<serde_json::Value>>,

    #[serde(rename = "pos")]
    pub pos: Vec<f64>,
}
