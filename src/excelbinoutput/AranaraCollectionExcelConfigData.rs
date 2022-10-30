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

pub type AranaraCollectionExcelConfigData = Vec<AranaraCollectionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AranaraCollectionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "CALPIGAKMIN")]
    pub calpigakmin: Calpigakmin,
}

#[derive(Serialize, Deserialize)]
pub enum Calpigakmin {
    #[serde(rename = "ARANARA_COLLECTION_TYPE_CATALOG_V1")]
    AranaraCollectionTypeCatalogV1,
}
