/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityChessMapPointExcelConfigData = Vec<ActivityChessMapPointExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityChessMapPointExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,
    pub coord_x: f64,
    pub coord_y: f64,
}
