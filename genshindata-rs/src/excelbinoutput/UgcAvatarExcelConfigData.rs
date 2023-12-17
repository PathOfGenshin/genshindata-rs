/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcAvatarExcelConfigData = Vec<UgcAvatarExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UgcAvatarExcelConfigDatum {
    pub avatar_id: i64,
    pub trial_avatar_id: i64,
    #[serde(rename = "FJOFLCHFJMC")]
    pub fjoflchfjmc: f64,
}
