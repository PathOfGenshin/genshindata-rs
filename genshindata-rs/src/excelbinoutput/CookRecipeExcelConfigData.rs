// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type CookRecipeExcelConfigData = Vec<CookRecipeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CookRecipeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "rankLevel")]
    pub rank_level: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "effectDesc")]
    pub effect_desc: Vec<i64>,

    #[serde(rename = "foodType")]
    pub food_type: FoodType,

    #[serde(rename = "cookMethod")]
    pub cook_method: CookMethod,

    #[serde(rename = "isDefaultUnlocked")]
    pub is_default_unlocked: Option<bool>,

    #[serde(rename = "maxProficiency")]
    pub max_proficiency: i64,

    #[serde(rename = "qualityOutputVec")]
    pub quality_output_vec: Vec<PutVec>,

    #[serde(rename = "inputVec")]
    pub input_vec: Vec<PutVec>,

    #[serde(rename = "qteParam")]
    pub qte_param: String,

    #[serde(rename = "qteQualityWeightVec")]
    pub qte_quality_weight_vec: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutVec {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CookMethod {
    #[serde(rename = "COOK_METHOD_BAKE")]
    CookMethodBake,

    #[serde(rename = "COOK_METHOD_BOIL")]
    CookMethodBoil,

    #[serde(rename = "COOK_METHOD_FRY")]
    CookMethodFry,

    #[serde(rename = "COOK_METHOD_STEAM")]
    CookMethodSteam,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FoodType {
    #[serde(rename = "COOK_FOOD_ATTACK")]
    CookFoodAttack,

    #[serde(rename = "COOK_FOOD_DEFENSE")]
    CookFoodDefense,

    #[serde(rename = "COOK_FOOD_FUNCTION")]
    CookFoodFunction,

    #[serde(rename = "COOK_FOOD_HEAL")]
    CookFoodHeal,
}

pub fn load() -> Result<CookRecipeExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "CookRecipeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}