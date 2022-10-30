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

pub type NpcFirstMetExcelConfigData = Vec<NpcFirstMetExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NpcFirstMetExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "subQuestIdList")]
    pub sub_quest_id_list: Vec<i64>,

    #[serde(rename = "avatarID")]
    pub avatar_id: i64,

    #[serde(rename = "avatarDescriptionTextMapHash")]
    pub avatar_description_text_map_hash: i64,
}
