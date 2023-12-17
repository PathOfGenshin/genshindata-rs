/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgScenePointExcelConfigData = Vec<GcgScenePointExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcgScenePointExcelConfigDatum {
    pub id: i64,
    pub pos: Vec<f64>,
    #[serde(rename = "CCFDKANCHBG")]
    pub ccfdkanchbg: f64,
    pub motion_type: Option<MotionType>,
    #[serde(rename = "JLMHJKFNPOM")]
    pub jlmhjkfnpom: Vec<Jlmhjkfnpom>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Jlmhjkfnpom {
    #[serde(rename = "SUPPORT_CALL")]
    SupportCall,
    #[serde(rename = "SUPPORT_NONE")]
    SupportNone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MotionType {
    #[serde(rename = "POINT_MOTION_SIT")]
    PointMotionSit,
    #[serde(rename = "POINT_MOTION_STAND")]
    PointMotionStand,
}
