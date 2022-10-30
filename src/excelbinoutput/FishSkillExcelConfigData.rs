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

pub type FishSkillExcelConfigData = Vec<FishSkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FishSkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "skillCategory")]
    pub skill_category: SkillCategory,

    #[serde(rename = "DCGGDMIPIAG")]
    pub dcggdmipiag: Dcggdmipiag,

    #[serde(rename = "param")]
    pub param: Vec<f64>,

    #[serde(rename = "INIEAHDPPAJ")]
    pub inieahdppaj: Option<f64>,

    #[serde(rename = "AJHFGLNAHFE")]
    pub ajhfglnahfe: Vec<f64>,

    #[serde(rename = "BMKABBDJOPN")]
    pub bmkabbdjopn: Vec<f64>,

    #[serde(rename = "HNKHPIEJGDD")]
    pub hnkhpiejgdd: Vec<i64>,

    #[serde(rename = "PFPHEDFFBOI")]
    pub pfphedffboi: Vec<f64>,

    #[serde(rename = "duration")]
    pub duration: f64,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "GGJKMPNMDGE")]
    pub ggjkmpnmdge: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Dcggdmipiag {
    #[serde(rename = "FISH_SKILL_HP")]
    FishSkillHp,

    #[serde(rename = "FISH_SKILL_TIME")]
    FishSkillTime,
}

#[derive(Serialize, Deserialize)]
pub enum SkillCategory {
    #[serde(rename = "FISH_SKILL_CATEGORY_BONUS")]
    FishSkillCategoryBonus,

    #[serde(rename = "FISH_SKILL_CATEGORY_FORCE")]
    FishSkillCategoryForce,
}
