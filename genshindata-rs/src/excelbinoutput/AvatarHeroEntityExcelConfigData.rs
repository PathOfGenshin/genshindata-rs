/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AvatarHeroEntityExcelConfigData = Vec<AvatarHeroEntityExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarHeroEntityExcelConfigDatum {
    pub avatar_id: i64,
    #[serde(rename = "KCANMNBNNED")]
    pub kcanmnbnned: f64,
    pub gacha_image_name_hash: f64,
}
