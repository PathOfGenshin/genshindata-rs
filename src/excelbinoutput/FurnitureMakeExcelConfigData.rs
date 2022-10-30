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

pub type FurnitureMakeExcelConfigData = Vec<FurnitureMakeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FurnitureMakeExcelConfigDatum {
    #[serde(rename = "configID")]
    pub config_id: i64,

    #[serde(rename = "furnitureItemID")]
    pub furniture_item_id: i64,

    #[serde(rename = "count")]
    pub count: i64,

    #[serde(rename = "exp")]
    pub exp: i64,

    #[serde(rename = "materialItems")]
    pub material_items: Vec<MaterialItem>,

    #[serde(rename = "maxAccelerateTime")]
    pub max_accelerate_time: i64,

    #[serde(rename = "makeTime")]
    pub make_time: i64,

    #[serde(rename = "quickFetchMaterialNum")]
    pub quick_fetch_material_num: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}
