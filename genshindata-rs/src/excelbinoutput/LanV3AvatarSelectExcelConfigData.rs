/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LanV3AvatarSelectExcelConfigData = Vec<LanV3AvatarSelectExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LanV3AvatarSelectExcelConfigDatum {
    #[serde(rename = "avatarId")]
    pub avatar_id: i64,
    pub pbpgbfgmoii: f64,
}
