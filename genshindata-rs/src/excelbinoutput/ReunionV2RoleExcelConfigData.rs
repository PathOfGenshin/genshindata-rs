/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ReunionV2RoleExcelConfigData = Vec<ReunionV2RoleExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ReunionV2RoleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "avatarId")]
    pub avatar_id: i64,
    pub ajocpklacnh: String,
    pub lnoigblcfma: String,
    pub oidehfhbbfj: String,
    pub afdegggjakb: Vec<i64>,
}
