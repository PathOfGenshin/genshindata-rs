/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MichiaeRadarExcelConfigData = Vec<MichiaeRadarExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MichiaeRadarExcelConfigDatum {
    pub mark_type: String,
    pub gadget_id_list: Vec<i64>,
}
