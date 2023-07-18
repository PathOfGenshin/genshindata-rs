/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type PhotographTaskData = Vec<PhotographTaskDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotographTaskDatum {
    pub task_id: i64,
    #[serde(rename = "sceneID")]
    pub scene_id: i64,
    pub center_x: f64,
    pub center_y: f64,
    pub center_z: f64,
    pub radius: i64,
    #[serde(rename = "targetGadgetID")]
    pub target_gadget_id: Vec<String>,
    pub end_time: i64,
    pub questid: i64,
    pub finish_tips_text_map_hash: i64,
    pub start_tips_text_map_hash: i64,
    pub start_time: Option<i64>,
    #[serde(rename = "LMPCJAMCJLN")]
    pub lmpcjamcjln: Option<Lmpcjamcjln>,
    #[serde(rename = "HOODMEOEGGC")]
    pub hoodmeoeggc: Option<i64>,
    #[serde(rename = "DDLKLHJPDEH")]
    pub ddlklhjpdeh: Option<i64>,
    #[serde(rename = "OCKLFJCKDIJ")]
    pub ocklfjckdij: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Lmpcjamcjln {
    #[serde(rename = "PHOTOGRAPH_TASK_TYPE_INTERACTION")]
    PhotographTaskTypeInteraction,
}
