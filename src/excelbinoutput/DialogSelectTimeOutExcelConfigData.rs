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

pub type DialogSelectTimeOutExcelConfigData = Vec<DialogSelectTimeOutExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DialogSelectTimeOutExcelConfigDatum {
    #[serde(rename = "_id")]
    pub id: i64,

    #[serde(rename = "_timeLimit")]
    pub time_limit: f64,

    #[serde(rename = "_nextDialogID")]
    pub next_dialog_id: i64,
}
