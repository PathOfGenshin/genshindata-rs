/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type NewActivityTimeGroupExcelConfigData = Vec<NewActivityTimeGroupExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewActivityTimeGroupExcelConfigDatum {
    pub id: i64,
    pub group_id_list: Vec<i64>,
    pub duration: Vec<i64>,
}
