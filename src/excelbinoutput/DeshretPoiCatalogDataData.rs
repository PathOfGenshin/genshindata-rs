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

pub type DeshretPoiCatalogDataData = Vec<DeshretPoiCatalogDataDatum>;

#[derive(Serialize, Deserialize)]
pub struct DeshretPoiCatalogDataDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LCMILPPPBCD")]
    pub lcmilpppbcd: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "OKJELPAKMLI")]
    pub okjelpakmli: i64,

    #[serde(rename = "LBBANBHMCMC")]
    pub lbbanbhmcmc: i64,

    #[serde(rename = "FLCOMLAGKID")]
    pub flcomlagkid: i64,
}
