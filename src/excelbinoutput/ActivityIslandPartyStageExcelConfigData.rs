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

pub type ActivityIslandPartyStageExcelConfigData = Vec<ActivityIslandPartyStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityIslandPartyStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "JFMIMMBHHNK")]
    pub jfmimmbhhnk: String,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "MFIPOMCNJMN")]
    pub mfipomcnjmn: i64,

    #[serde(rename = "matchId")]
    pub match_id: i64,

    #[serde(rename = "draftId")]
    pub draft_id: i64,

    #[serde(rename = "HAFOEGJLOAK")]
    pub hafoegjloak: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "CLCOAOMOANE")]
    pub clcoaomoane: i64,

    #[serde(rename = "KNKEFFFJKMD")]
    pub knkefffjkmd: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "KJJAPBBALHH")]
    pub kjjapbbalhh: Vec<i64>,

    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
}
