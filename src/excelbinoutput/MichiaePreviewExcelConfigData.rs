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
use std::collections::HashMap;

pub type MichiaePreviewExcelConfigData = Vec<MichiaePreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MichiaePreviewExcelConfigDatum {
    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KGEAFDLGPFI")]
    pub kgeafdlgpfi: i64,

    #[serde(rename = "CKOHFOOOKOG")]
    pub ckohfoookog: i64,

    #[serde(rename = "rewardPreviewID")]
    pub reward_preview_id: i64,

    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,

    #[serde(rename = "MEDLKFNLFPM")]
    pub medlkfnlfpm: i64,

    #[serde(rename = "NKPPJKPJBCF")]
    pub nkppjkpjbcf: f64,

    #[serde(rename = "MFPENBLLMMJ")]
    pub mfpenbllmmj: f64,

    #[serde(rename = "OJHJJOIJPGA")]
    pub ojhjjoijpga: i64,

    #[serde(rename = "JOCPGPMKOIE")]
    pub jocpgpmkoie: i64,

    #[serde(rename = "GHBCECBAIHL")]
    pub ghbcecbaihl: i64,

    #[serde(rename = "DHNBKBFLIHJ")]
    pub dhnbkbflihj: i64,

    #[serde(rename = "PNENAOPAPOM")]
    pub pnenaopapom: HashMap<String, i64>,

    #[serde(rename = "OEGMFOHBAHC")]
    pub oegmfohbahc: Vec<i64>,
}
