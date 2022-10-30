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

pub type MusicRiddleConfigData = Vec<MusicRiddleConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MusicRiddleConfigDatum {
    #[serde(rename = "LNCMLPKLEDG")]
    pub lncmlpkledg: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "MCCIJIPGJLD")]
    pub mccijipgjld: i64,

    #[serde(rename = "NLJDBCFJOBJ")]
    pub nljdbcfjobj: Vec<i64>,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "JILJDJKFJFL")]
    pub jiljdjkfjfl: Option<i64>,

    #[serde(rename = "HALGLNBAGIJ")]
    pub halglnbagij: Option<i64>,

    #[serde(rename = "GADIOHMEDHC")]
    pub gadiohmedhc: Option<bool>,
}
