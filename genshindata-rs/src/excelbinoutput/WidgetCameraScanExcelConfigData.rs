/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WidgetCameraScanExcelConfigData = Vec<WidgetCameraScanExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetCameraScanExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "cameraID")]
    pub camera_id: i64,
    #[serde(rename = "configID")]
    pub config_id: i64,
    pub scannable_state: Vec<i64>,
    pub is_hint: bool,
    pub action: String,
}
