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

pub type CustomLevelGroupConfigData = Vec<CustomLevelGroupConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CustomLevelGroupConfigDatum {
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "dungeonList")]
    pub dungeon_list: Vec<i64>,

    #[serde(rename = "MKFDPBNODIC")]
    pub mkfdpbnodic: Vec<i64>,

    #[serde(rename = "BNABDDAIICI")]
    pub bnabddaiici: i64,

    #[serde(rename = "PJOFPGGKFGE")]
    pub pjofpggkfge: i64,

    #[serde(rename = "MIKLLOAGLAH")]
    pub miklloaglah: i64,

    #[serde(rename = "HGPBBAEGCDJ")]
    pub hgpbbaegcdj: i64,

    #[serde(rename = "NHMAJLPODGD")]
    pub nhmajlpodgd: i64,

    #[serde(rename = "AGKJDOAPNJI")]
    pub agkjdoapnji: i64,
}
