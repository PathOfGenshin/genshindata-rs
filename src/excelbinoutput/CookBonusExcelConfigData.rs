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

pub type CookBonusExcelConfigData = Vec<CookBonusExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CookBonusExcelConfigDatum {
    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "recipeId")]
    pub recipe_id: i64,

    #[serde(rename = "bonusType")]
    pub bonus_type: BonusType,

    #[serde(rename = "paramVec")]
    pub param_vec: Vec<i64>,

    #[serde(rename = "complexParamVec")]
    pub complex_param_vec: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum BonusType {
    #[serde(rename = "COOK_BONUS_REPLACE")]
    CookBonusReplace,
}
