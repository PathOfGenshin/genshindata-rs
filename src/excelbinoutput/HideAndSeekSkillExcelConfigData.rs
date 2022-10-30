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

pub type HideAndSeekSkillExcelConfigData = Vec<HideAndSeekSkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HideAndSeekSkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "skillID")]
    pub skill_id: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "skillCategory")]
    pub skill_category: SkillCategory,

    #[serde(rename = "skillSubCategory")]
    pub skill_sub_category: SkillSubCategory,

    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,

    #[serde(rename = "categoryDescTextMapHash")]
    pub category_desc_text_map_hash: i64,

    #[serde(rename = "HDCCEKCNNAH")]
    pub hdccekcnnah: i64,

    #[serde(rename = "APDEPHNBJBK")]
    pub apdephnbjbk: i64,
}

#[derive(Serialize, Deserialize)]
pub enum SkillCategory {
    #[serde(rename = "HIDE_AND_SEEK_SKILL_CATEGORY_HUNTER")]
    HideAndSeekSkillCategoryHunter,

    #[serde(rename = "HIDE_AND_SEEK_SKILL_CATEGORY_PREY")]
    HideAndSeekSkillCategoryPrey,
}

#[derive(Serialize, Deserialize)]
pub enum SkillSubCategory {
    #[serde(rename = "HIDE_AND_SEEK_SKILL_SUB_CATEGORY_COMMON")]
    HideAndSeekSkillSubCategoryCommon,

    #[serde(rename = "HIDE_AND_SEEK_SKILL_SUB_CATEGORY_SKILL1")]
    HideAndSeekSkillSubCategorySkill1,

    #[serde(rename = "HIDE_AND_SEEK_SKILL_SUB_CATEGORY_SKILL2")]
    HideAndSeekSkillSubCategorySkill2,
}
