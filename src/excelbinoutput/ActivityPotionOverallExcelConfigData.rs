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

pub type ActivityPotionOverallExcelConfigData = Vec<ActivityPotionOverallExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityPotionOverallExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "ODBENMKBIBJ")]
    pub odbenmkbibj: f64,

    #[serde(rename = "MKHPHMHKIDB")]
    pub mkhphmhkidb: f64,

    #[serde(rename = "CEDNCANGMJP")]
    pub cedncangmjp: f64,
}
