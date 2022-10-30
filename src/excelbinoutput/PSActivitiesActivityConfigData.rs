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

pub type PsActivitiesActivityConfigData = Vec<PsActivitiesActivityConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PsActivitiesActivityConfigDatum {
    #[serde(rename = "activityNameTextMapHash")]
    pub activity_name_text_map_hash: i64,

    #[serde(rename = "activityDescTextMapHash")]
    pub activity_desc_text_map_hash: i64,

    #[serde(rename = "availableByDefault")]
    pub available_by_default: Option<bool>,

    #[serde(rename = "isRequiredForCompletion")]
    pub is_required_for_completion: bool,

    #[serde(rename = "largeIcon")]
    pub large_icon: String,

    #[serde(rename = "smallIcon")]
    pub small_icon: String,

    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "objectID")]
    pub object_id: String,

    #[serde(rename = "FIBLOAJMPJG")]
    pub fibloajmpjg: String,
}
