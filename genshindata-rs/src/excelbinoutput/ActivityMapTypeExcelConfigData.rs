/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityMapTypeExcelConfigData = Vec<ActivityMapTypeExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityMapTypeExcelConfigDatum {
    #[serde(rename = "type")]
    pub activity_map_type_excel_config_datum_type: String,
    #[serde(rename = "GJFMNFEDNCF")]
    pub gjfmnfedncf: String,
}
