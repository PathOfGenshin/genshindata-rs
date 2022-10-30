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

pub type LimitRegionExcelConfigData = Vec<LimitRegionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LimitRegionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "type")]
    pub limit_region_excel_config_datum_type: Type,

    #[serde(rename = "ADCPJOMDBBN")]
    pub adcpjomdbbn: Option<String>,

    #[serde(rename = "order")]
    pub order: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "LIMIT_REGION_TYPE_ACTIVITY")]
    LimitRegionTypeActivity,

    #[serde(rename = "LIMIT_REGION_TYPE_BIGWORLD")]
    LimitRegionTypeBigworld,

    #[serde(rename = "LIMIT_REGION_TYPE_HOMEWORLD")]
    LimitRegionTypeHomeworld,
}
