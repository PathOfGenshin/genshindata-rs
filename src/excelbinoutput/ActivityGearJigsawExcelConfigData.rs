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

pub type ActivityGearJigsawExcelConfigData = Vec<ActivityGearJigsawExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityGearJigsawExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MPJOBAMPMPP")]
    pub mpjobampmpp: i64,

    #[serde(rename = "OBGJFOPJABK")]
    pub obgjfopjabk: String,

    #[serde(rename = "IPKALPKLINE")]
    pub ipkalpkline: String,

    #[serde(rename = "FCHHLHBLIOH")]
    pub fchhlhblioh: String,

    #[serde(rename = "KEKGLAJJJIE")]
    pub kekglajjjie: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,
}
