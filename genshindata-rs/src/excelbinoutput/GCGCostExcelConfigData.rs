/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgCostExcelConfigData = Vec<GcgCostExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcgCostExcelConfigDatum {
    #[serde(rename = "type")]
    pub gcg_cost_excel_config_datum_type: String,
    #[serde(rename = "EJOPIHNDDIA")]
    pub ejopihnddia: i64,
}
