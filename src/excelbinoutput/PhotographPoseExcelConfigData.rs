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

pub type PhotographPoseExcelConfigData = Vec<PhotographPoseExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PhotographPoseExcelConfigDatum {
    #[serde(rename = "characterId")]
    pub character_id: i64,

    #[serde(rename = "poseId")]
    pub pose_id: Vec<i64>,
}
