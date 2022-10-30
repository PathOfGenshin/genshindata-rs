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

pub type ChannellerSlabLoopDungeonPreviewExcelConfigData =
    Vec<ChannellerSlabLoopDungeonPreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChannellerSlabLoopDungeonPreviewExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "entryDescTextMapHash")]
    pub entry_desc_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "pointID")]
    pub point_id: i64,
}
