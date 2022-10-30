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

pub type ActivityPotionStageExcelConfigData = Vec<ActivityPotionStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityPotionStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "ECJLPFICKPL")]
    pub ecjlpfickpl: Vec<i64>,

    #[serde(rename = "GNIJIEHGBDA")]
    pub gnijiehgbda: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "BNHKIIIIFMG")]
    pub bnhkiiiifmg: Vec<i64>,

    #[serde(rename = "BDEEPDFHPIL")]
    pub bdeepdfhpil: Vec<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "LONCNOONFJO")]
    pub loncnoonfjo: i64,
}
