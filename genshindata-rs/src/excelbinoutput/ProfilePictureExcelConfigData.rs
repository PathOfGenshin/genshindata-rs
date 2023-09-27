/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ProfilePictureExcelConfigData = Vec<ProfilePictureExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePictureExcelConfigDatum {
    pub id: i64,
    pub icon_path: String,
    pub priority: i64,
    pub name_text_map_hash: i64,
    #[serde(rename = "FDDOKGDIKCM")]
    pub fddokgdikcm: Fddokgdikcm,
    #[serde(rename = "CBKKABADBBK")]
    pub cbkkabadbbk: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Fddokgdikcm {
    #[serde(rename = "PROFILE_PICTURE_UNLOCK_BY_AVATAR")]
    ProfilePictureUnlockByAvatar,
    #[serde(rename = "PROFILE_PICTURE_UNLOCK_BY_COSTUME")]
    ProfilePictureUnlockByCostume,
    #[serde(rename = "PROFILE_PICTURE_UNLOCK_BY_ITEM")]
    ProfilePictureUnlockByItem,
}
