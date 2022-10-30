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

pub type MonsterAffixExcelConfigData = Vec<MonsterAffixExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MonsterAffixExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "affix")]
    pub affix: String,

    #[serde(rename = "comment")]
    pub comment: String,

    #[serde(rename = "abilityName")]
    pub ability_name: Vec<String>,

    #[serde(rename = "isLegal")]
    pub is_legal: IsLegal,

    #[serde(rename = "iconPath")]
    pub icon_path: IconPath,

    #[serde(rename = "generalSkillIcon")]
    pub general_skill_icon: String,

    #[serde(rename = "isCommon")]
    pub is_common: Option<bool>,

    #[serde(rename = "preAdd")]
    pub pre_add: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum IconPath {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Skill_E_Ambor")]
    SkillEAmbor,

    #[serde(rename = "Skill_E_Ayaka")]
    SkillEAyaka,

    #[serde(rename = "Skill_E_Diluc_01")]
    SkillEDiluc01,

    #[serde(rename = "Skill_E_Kate")]
    SkillEKate,

    #[serde(rename = "Skill_E_PlayerWind_01")]
    SkillEPlayerWind01,

    #[serde(rename = "Skill_S_Diluc_01_01")]
    SkillSDiluc01_01,

    #[serde(rename = "Skill_S_Diluc_01_02")]
    SkillSDiluc01_02,

    #[serde(rename = "Skill_S_Diluc_01_03")]
    SkillSDiluc01_03,

    #[serde(rename = "Skill_S_Qin_01")]
    SkillSQin01,
}

#[derive(Serialize, Deserialize)]
pub enum IsLegal {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "是")]
    IsLegal,
}
