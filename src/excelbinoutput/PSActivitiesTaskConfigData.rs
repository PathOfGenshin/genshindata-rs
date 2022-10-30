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

pub type PsActivitiesTaskConfigData = Vec<PsActivitiesTaskConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PsActivitiesTaskConfigDatum {
    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "taskNameTextMapHash")]
    pub task_name_text_map_hash: i64,

    #[serde(rename = "isRequiredForCompletion")]
    pub is_required_for_completion: bool,

    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "objectID")]
    pub object_id: String,

    #[serde(rename = "FIBLOAJMPJG")]
    pub fibloajmpjg: String,

    #[serde(rename = "hidden")]
    pub hidden: Option<bool>,
}
