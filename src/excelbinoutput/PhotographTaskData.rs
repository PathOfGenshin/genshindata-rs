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

pub type PhotographTaskData = Vec<PhotographTaskDatum>;

#[derive(Serialize, Deserialize)]
pub struct PhotographTaskDatum {
    #[serde(rename = "taskId")]
    pub task_id: i64,

    #[serde(rename = "sceneID")]
    pub scene_id: i64,

    #[serde(rename = "centerX")]
    pub center_x: f64,

    #[serde(rename = "centerY")]
    pub center_y: f64,

    #[serde(rename = "centerZ")]
    pub center_z: f64,

    #[serde(rename = "radius")]
    pub radius: f64,

    #[serde(rename = "targetGadgetID")]
    pub target_gadget_id: Vec<String>,

    #[serde(rename = "endTime")]
    pub end_time: f64,

    #[serde(rename = "questid")]
    pub questid: i64,

    #[serde(rename = "finishTipsTextMapHash")]
    pub finish_tips_text_map_hash: i64,

    #[serde(rename = "startTipsTextMapHash")]
    pub start_tips_text_map_hash: i64,

    #[serde(rename = "startTime")]
    pub start_time: Option<f64>,

    #[serde(rename = "DDOIKDALOIB")]
    pub ddoikdaloib: Option<Ddoikdaloib>,
}

#[derive(Serialize, Deserialize)]
pub enum Ddoikdaloib {
    #[serde(rename = "PHOTOGRAPH_TASK_TYPE_INTERACTION")]
    PhotographTaskTypeInteraction,
}
