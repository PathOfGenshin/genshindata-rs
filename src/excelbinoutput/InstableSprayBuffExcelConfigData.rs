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

pub type InstableSprayBuffExcelConfigData = Vec<InstableSprayBuffExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InstableSprayBuffExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "GPDAEIMDCBF")]
    pub gpdaeimdcbf: String,

    #[serde(rename = "AMMJJJIODCM")]
    pub ammjjjiodcm: String,

    #[serde(rename = "NBLOCEJHFCN")]
    pub nblocejhfcn: i64,

    #[serde(rename = "HPIKALMBHLL")]
    pub hpikalmbhll: i64,

    #[serde(rename = "elementType")]
    pub element_type: i64,

    #[serde(rename = "buffNameTextMapHash")]
    pub buff_name_text_map_hash: i64,

    #[serde(rename = "CPOAMLCBFAG")]
    pub cpoamlcbfag: i64,

    #[serde(rename = "JGOAAIOIKNG")]
    pub jgoaaioikng: Vec<String>,
}
