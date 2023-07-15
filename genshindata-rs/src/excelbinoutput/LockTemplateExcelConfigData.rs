/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LockTemplateExcelConfigData = Vec<LockTemplateExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockTemplateExcelConfigDatum {
    #[serde(rename = "type")]
    pub lock_template_excel_config_datum_type: String,
    pub range: f64,
    pub normal_pri: Option<f64>,
    pub combat_pri: Option<f64>,
}
