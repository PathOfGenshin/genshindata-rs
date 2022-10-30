// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type HideAndSeekSkillExcelConfigData = Vec<HideAndSeekSkillExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum SkillCategory {
    #[serde(rename = "HIDE_AND_SEEK_SKILL_CATEGORY_HUNTER")]
    HideAndSeekSkillCategoryHunter,

    #[serde(rename = "HIDE_AND_SEEK_SKILL_CATEGORY_PREY")]
    HideAndSeekSkillCategoryPrey,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SkillSubCategory {
    #[serde(rename = "HIDE_AND_SEEK_SKILL_SUB_CATEGORY_COMMON")]
    HideAndSeekSkillSubCategoryCommon,

    #[serde(rename = "HIDE_AND_SEEK_SKILL_SUB_CATEGORY_SKILL1")]
    HideAndSeekSkillSubCategorySkill1,

    #[serde(rename = "HIDE_AND_SEEK_SKILL_SUB_CATEGORY_SKILL2")]
    HideAndSeekSkillSubCategorySkill2,
}

pub fn load() -> Result<HideAndSeekSkillExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "HideAndSeekSkillExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
