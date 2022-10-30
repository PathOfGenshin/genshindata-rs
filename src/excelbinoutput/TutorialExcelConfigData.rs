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

pub type TutorialExcelConfigData = Vec<TutorialExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TutorialExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "detailIdList")]
    pub detail_id_list: Vec<i64>,

    #[serde(rename = "LPJENIADFLM")]
    pub lpjeniadflm: Vec<i64>,

    #[serde(rename = "OFLAMIMLKBC")]
    pub oflamimlkbc: Vec<i64>,

    #[serde(rename = "pauseGame")]
    pub pause_game: Option<bool>,

    #[serde(rename = "KOLNPCDKNCD")]
    pub kolnpcdkncd: Option<bool>,
}
