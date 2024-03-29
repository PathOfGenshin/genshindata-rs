/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BirthdayMailExcelConfigData = Vec<BirthdayMailExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BirthdayMailExcelConfigDatum {
    pub id: i64,
    pub mail_id: i64,
    pub reward_id: i64,
    pub effective_date: String,
    #[serde(rename = "LGDKLOMEDMI")]
    pub lgdklomedmi: i64,
}
