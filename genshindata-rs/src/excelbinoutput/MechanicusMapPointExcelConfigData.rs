/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MechanicusMapPointExcelConfigData = Vec<MechanicusMapPointExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MechanicusMapPointExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,
    pub coord_x: f64,
    pub coord_y: f64,
}
