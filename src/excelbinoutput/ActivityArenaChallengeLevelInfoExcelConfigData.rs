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

pub type ActivityArenaChallengeLevelInfoExcelConfigData =
    Vec<ActivityArenaChallengeLevelInfoExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityArenaChallengeLevelInfoExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "monsterPreviewId")]
    pub monster_preview_id: i64,

    #[serde(rename = "monsterConfig")]
    pub monster_config: String,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "NGNPIMJPNCD")]
    pub ngnpimjpncd: String,

    #[serde(rename = "PBDPAAAMIPO")]
    pub pbdpaaamipo: i64,

    #[serde(rename = "CDPCCEDAHGH")]
    pub cdpccedahgh: String,

    #[serde(rename = "EIPEBFPDKLB")]
    pub eipebfpdklb: i64,

    #[serde(rename = "NKMCJAEGBAB")]
    pub nkmcjaegbab: String,

    #[serde(rename = "levelDetailDescTextMapHash")]
    pub level_detail_desc_text_map_hash: i64,

    #[serde(rename = "challengeIdList")]
    pub challenge_id_list: Vec<i64>,
}
