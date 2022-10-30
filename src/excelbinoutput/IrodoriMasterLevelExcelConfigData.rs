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

pub type IrodoriMasterLevelExcelConfigData = Vec<IrodoriMasterLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct IrodoriMasterLevelExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "MPEIIPLLCNO")]
    pub mpeiipllcno: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "AFIKCBKCBPO")]
    pub afikcbkcbpo: String,

    #[serde(rename = "BNHKIIIIFMG")]
    pub bnhkiiiifmg: Vec<i64>,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
}
