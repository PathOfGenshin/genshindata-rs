/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FetterInfoExcelConfigData = Vec<FetterInfoExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetterInfoExcelConfigDatum {
    pub info_birth_month: Option<i64>,
    pub info_birth_day: Option<i64>,
    pub avatar_native_text_map_hash: i64,
    pub avatar_vision_befor_text_map_hash: i64,
    pub avatar_constellation_befor_text_map_hash: i64,
    pub avatar_title_text_map_hash: i64,
    pub avatar_detail_text_map_hash: i64,
    pub avatar_assoc_type: AvatarAssocType,
    pub cv_chinese_text_map_hash: i64,
    pub cv_japanese_text_map_hash: i64,
    pub cv_english_text_map_hash: i64,
    pub cv_korean_text_map_hash: i64,
    pub avatar_vision_after_text_map_hash: i64,
    pub avatar_constellation_after_text_map_hash: i64,
    #[serde(rename = "MNFCJJCOKAL")]
    pub mnfcjjcokal: i64,
    #[serde(rename = "LIAGKAGANIN")]
    pub liagkaganin: i64,
    pub fetter_id: i64,
    pub avatar_id: i64,
    pub open_conds: Vec<Option<serde_json::Value>>,
    pub finish_conds: Vec<FinishCond>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AvatarAssocType {
    #[serde(rename = "ASSOC_TYPE_FATUI")]
    AssocTypeFatui,
    #[serde(rename = "ASSOC_TYPE_FONTAINE")]
    AssocTypeFontaine,
    #[serde(rename = "ASSOC_TYPE_INAZUMA")]
    AssocTypeInazuma,
    #[serde(rename = "ASSOC_TYPE_LIYUE")]
    AssocTypeLiyue,
    #[serde(rename = "ASSOC_TYPE_MAINACTOR")]
    AssocTypeMainactor,
    #[serde(rename = "ASSOC_TYPE_MONDSTADT")]
    AssocTypeMondstadt,
    #[serde(rename = "ASSOC_TYPE_RANGER")]
    AssocTypeRanger,
    #[serde(rename = "ASSOC_TYPE_SUMERU")]
    AssocTypeSumeru,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishCond {
    pub cond_type: CondType,
    pub param_list: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CondType {
    #[serde(rename = "FETTER_COND_AVATAR_LEVEL")]
    FetterCondAvatarLevel,
    #[serde(rename = "FETTER_COND_FINISH_QUEST")]
    FetterCondFinishQuest,
    #[serde(rename = "FETTER_COND_NOT_OPEN")]
    FetterCondNotOpen,
}
