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

pub type MusicGamePreviewConfigData = Vec<MusicGamePreviewConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MusicGamePreviewConfigDatum {
    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "ABGHENDDOMD")]
    pub abghenddomd: Vec<i64>,

    #[serde(rename = "GNDHBNHAFJH")]
    pub gndhbnhafjh: i64,

    #[serde(rename = "DJKCKMEMANA")]
    pub djkckmemana: i64,

    #[serde(rename = "DENJIMBDDNL")]
    pub denjimbddnl: i64,
}
