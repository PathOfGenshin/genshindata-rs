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

pub type CatalogExcelConfigData = Vec<CatalogExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CatalogExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub catalog_excel_config_datum_type: String,

    #[serde(rename = "NEAGDHPBCGF")]
    pub neagdhpbcgf: Vec<Vec<i64>>,

    #[serde(rename = "JPBIPKDNPEO")]
    pub jpbipkdnpeo: Vec<Jpbipkdnpeo>,

    #[serde(rename = "GBLHPLPAKGE")]
    pub gblhplpakge: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Jpbipkdnpeo {
    #[serde(rename = "DHBMIENBFMD")]
    pub dhbmienbfmd: Option<i64>,

    #[serde(rename = "LKOAEFLJCEC")]
    pub lkoaefljcec: Option<i64>,
}
