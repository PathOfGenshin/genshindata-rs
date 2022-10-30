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

pub type MusicGameBasicConfigData = Vec<MusicGameBasicConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MusicGameBasicConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "musicID")]
    pub music_id: i64,

    #[serde(rename = "musicLevel")]
    pub music_level: i64,

    #[serde(rename = "jsonPath")]
    pub json_path: String,

    #[serde(rename = "GJJEGJIHABK")]
    pub gjjegjihabk: i64,

    #[serde(rename = "successLatePoint")]
    pub success_late_point: i64,

    #[serde(rename = "scoreLevelList")]
    pub score_level_list: Vec<i64>,

    #[serde(rename = "scoreOneKey")]
    pub score_one_key: i64,

    #[serde(rename = "AJHHAFCKKCI")]
    pub ajhhafckkci: i64,

    #[serde(rename = "LPHNJCHNMFH")]
    pub lphnjchnmfh: i64,

    #[serde(rename = "AFPIAFGIBID")]
    pub afpiafgibid: f64,

    #[serde(rename = "ECFCJEEENPL")]
    pub ecfcjeeenpl: HashMap<String, f64>,

    #[serde(rename = "comboConfig")]
    pub combo_config: Vec<i64>,

    #[serde(rename = "scaleRange")]
    pub scale_range: i64,

    #[serde(rename = "CNNGOBKHIMN")]
    pub cnngobkhimn: String,

    #[serde(rename = "FJLLKMNCFEH")]
    pub fjllkmncfeh: String,

    #[serde(rename = "AMNHELPNKNO")]
    pub amnhelpnkno: i64,

    #[serde(rename = "BNDBECNBEOK")]
    pub bndbecnbeok: Vec<Bndbecnbeok>,

    #[serde(rename = "unlockTipsTextMapHash")]
    pub unlock_tips_text_map_hash: i64,

    #[serde(rename = "CHPLIFGKJDB")]
    pub chplifgkjdb: Vec<Bndbecnbeok>,
}

#[derive(Serialize, Deserialize)]
pub struct Bndbecnbeok {}
