/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AkaFesArchaeologyExcelConfigData = Vec<AkaFesArchaeologyExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AkaFesArchaeologyExcelConfigDatum {
    pub id: i64,
    pub end_quest_id: i64,
    #[serde(rename = "GGINAAHIFBF")]
    pub gginaahifbf: i64,
}
