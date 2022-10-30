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

pub type FindHilichurlHiliWeiExcelConfigData = Vec<FindHilichurlHiliWeiExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FindHilichurlHiliWeiExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "durationHint")]
    pub duration_hint: String,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "positionCenter")]
    pub position_center: Vec<f64>,

    #[serde(rename = "positionRadius")]
    pub position_radius: i64,

    #[serde(rename = "watcherID")]
    pub watcher_id: i64,
}
