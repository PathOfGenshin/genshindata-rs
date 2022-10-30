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

pub type IrodoriFlowerThemeExcelConfigData = Vec<IrodoriFlowerThemeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct IrodoriFlowerThemeExcelConfigDatum {
    #[serde(rename = "DEABKCHGODA")]
    pub deabkchgoda: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "HAFOEGJLOAK")]
    pub hafoegjloak: i64,

    #[serde(rename = "JILNEOHCOHP")]
    pub jilneohcohp: i64,

    #[serde(rename = "JMOEDAINFKE")]
    pub jmoedainfke: i64,

    #[serde(rename = "OLFLKDAEKJH")]
    pub olflkdaekjh: i64,

    #[serde(rename = "JEFOCCNIOBO")]
    pub jefoccniobo: i64,

    #[serde(rename = "JJAJOAPHBGE")]
    pub jjajoaphbge: i64,

    #[serde(rename = "LKBHFCIPKJJ")]
    pub lkbhfcipkjj: f64,

    #[serde(rename = "JKKBPINOPKJ")]
    pub jkkbpinopkj: Vec<i64>,

    #[serde(rename = "PKIIGMLFEKJ")]
    pub pkiigmlfekj: i64,

    #[serde(rename = "JPJGEHMDOFI")]
    pub jpjgehmdofi: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,
}
