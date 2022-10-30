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

pub type PsActivitiesSubTaskConfigData = Vec<PsActivitiesSubTaskConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PsActivitiesSubTaskConfigDatum {
    #[serde(rename = "taskID")]
    pub task_id: i64,

    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "objectID")]
    pub object_id: String,

    #[serde(rename = "FIBLOAJMPJG")]
    pub fibloajmpjg: Fibloajmpjg,

    #[serde(rename = "hidden")]
    pub hidden: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum Fibloajmpjg {
    #[serde(rename = "1.5.0")]
    The150,

    #[serde(rename = "1.6.0")]
    The160,

    #[serde(rename = "2.0.0")]
    The200,

    #[serde(rename = "2.1.0")]
    The210,

    #[serde(rename = "2.6.0")]
    The260,

    #[serde(rename = "3.0.0")]
    The300,

    #[serde(rename = "3.1.0")]
    The310,
}
