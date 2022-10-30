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

pub type CombineExcelConfigData = Vec<CombineExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CombineExcelConfigDatum {
    #[serde(rename = "combineId")]
    pub combine_id: i64,

    #[serde(rename = "playerLevel")]
    pub player_level: Option<i64>,

    #[serde(rename = "isDefaultShow")]
    pub is_default_show: Option<bool>,

    #[serde(rename = "combineType")]
    pub combine_type: i64,

    #[serde(rename = "subCombineType")]
    pub sub_combine_type: i64,

    #[serde(rename = "resultItemId")]
    pub result_item_id: i64,

    #[serde(rename = "resultItemCount")]
    pub result_item_count: i64,

    #[serde(rename = "scoinCost")]
    pub scoin_cost: Option<i64>,

    #[serde(rename = "randomItems")]
    pub random_items: Vec<RandomItem>,

    #[serde(rename = "materialItems")]
    pub material_items: Vec<MaterialItem>,

    #[serde(rename = "effectDescTextMapHash")]
    pub effect_desc_text_map_hash: i64,

    #[serde(rename = "recipeType")]
    pub recipe_type: RecipeType,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct RandomItem {
    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum RecipeType {
    #[serde(rename = "RECIPE_TYPE_COMBINE")]
    RecipeTypeCombine,

    #[serde(rename = "RECIPE_TYPE_COMBINE_HOMEWORLD")]
    RecipeTypeCombineHomeworld,

    #[serde(rename = "RECIPE_TYPE_CONVERT")]
    RecipeTypeConvert,
}
