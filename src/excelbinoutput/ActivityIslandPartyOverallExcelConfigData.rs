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

pub type ActivityIslandPartyOverallExcelConfigData =
    Vec<ActivityIslandPartyOverallExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityIslandPartyOverallExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "GBBPHBGKPLL")]
    pub gbbphbgkpll: i64,

    #[serde(rename = "IANJDDPIPOK")]
    pub ianjddpipok: i64,
}
