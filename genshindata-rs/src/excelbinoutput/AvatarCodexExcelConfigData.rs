/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AvatarCodexExcelConfigData = Vec<AvatarCodexExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarCodexExcelConfigDatum {
    pub sort_id: i64,
    pub sort_factor: i64,
    pub avatar_id: i64,
    pub begin_time: String,
    #[serde(rename = "GMKPEHDPAOD")]
    pub gmkpehdpaod: Option<bool>,
}
