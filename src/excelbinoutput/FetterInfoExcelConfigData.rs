// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type FetterInfoExcelConfigData = Vec<FetterInfoExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FetterInfoExcelConfigDatum {
    #[serde(rename = "infoBirthMonth")]
    pub info_birth_month: Option<i64>,

    #[serde(rename = "infoBirthDay")]
    pub info_birth_day: Option<i64>,

    #[serde(rename = "avatarNativeTextMapHash")]
    pub avatar_native_text_map_hash: i64,

    #[serde(rename = "avatarVisionBeforTextMapHash")]
    pub avatar_vision_befor_text_map_hash: i64,

    #[serde(rename = "avatarConstellationBeforTextMapHash")]
    pub avatar_constellation_befor_text_map_hash: i64,

    #[serde(rename = "avatarTitleTextMapHash")]
    pub avatar_title_text_map_hash: i64,

    #[serde(rename = "avatarDetailTextMapHash")]
    pub avatar_detail_text_map_hash: i64,

    #[serde(rename = "avatarAssocType")]
    pub avatar_assoc_type: AvatarAssocType,

    #[serde(rename = "cvChineseTextMapHash")]
    pub cv_chinese_text_map_hash: i64,

    #[serde(rename = "cvJapaneseTextMapHash")]
    pub cv_japanese_text_map_hash: i64,

    #[serde(rename = "cvEnglishTextMapHash")]
    pub cv_english_text_map_hash: i64,

    #[serde(rename = "cvKoreanTextMapHash")]
    pub cv_korean_text_map_hash: i64,

    #[serde(rename = "avatarVisionAfterTextMapHash")]
    pub avatar_vision_after_text_map_hash: i64,

    #[serde(rename = "avatarConstellationAfterTextMapHash")]
    pub avatar_constellation_after_text_map_hash: i64,

    #[serde(rename = "fetterId")]
    pub fetter_id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "openConds")]
    pub open_conds: Vec<Option<serde_json::Value>>,

    #[serde(rename = "KIENFJBHKEP")]
    pub kienfjbhkep: Vec<Kienfjbhkep>,
}

#[derive(Serialize, Deserialize)]
pub struct Kienfjbhkep {
    #[serde(rename = "condType")]
    pub cond_type: CondType,

    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum AvatarAssocType {
    #[serde(rename = "ASSOC_TYPE_FATUI")]
    AssocTypeFatui,

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

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "FETTER_COND_AVATAR_LEVEL")]
    FetterCondAvatarLevel,

    #[serde(rename = "FETTER_COND_FINISH_QUEST")]
    FetterCondFinishQuest,

    #[serde(rename = "FETTER_COND_NOT_OPEN")]
    FetterCondNotOpen,
}
