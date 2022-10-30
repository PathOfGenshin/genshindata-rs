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

pub type LanV2FireworksChallengeDataExcelConfigData =
    Vec<LanV2FireworksChallengeDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LanV2FireworksChallengeDataExcelConfigDatum {
    #[serde(rename = "challengeId")]
    pub challenge_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "KKEDINHDBMA")]
    pub kkedinhdbma: Vec<i64>,

    #[serde(rename = "KBEGEDAGNOI")]
    pub kbegedagnoi: i64,

    #[serde(rename = "EIFDFGELJBH")]
    pub eifdfgeljbh: i64,

    #[serde(rename = "ADDMBJEJANG")]
    pub addmbjejang: i64,

    #[serde(rename = "LBEKIAFPGAL")]
    pub lbekiafpgal: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "CCKJGHCBNOC")]
    pub cckjghcbnoc: i64,
}
