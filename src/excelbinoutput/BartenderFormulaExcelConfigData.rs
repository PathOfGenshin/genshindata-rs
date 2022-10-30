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

pub type BartenderFormulaExcelConfigData = Vec<BartenderFormulaExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BartenderFormulaExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LDGFABEMCDK")]
    pub ldgfabemcdk: Vec<Ialdnmdkabo>,

    #[serde(rename = "IALDNMDKABO")]
    pub ialdnmdkabo: Vec<Ialdnmdkabo>,

    #[serde(rename = "FIPEBBKFEIJ")]
    pub fipebbkfeij: Vec<i64>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "MEHNNNCLHLI")]
    pub mehnnnclhli: Option<i64>,

    #[serde(rename = "MBPKCLOODNN")]
    pub mbpkcloodnn: Option<i64>,

    #[serde(rename = "IKJBFINLMDD")]
    pub ikjbfinlmdd: i64,

    #[serde(rename = "NPEPHCHEMCC")]
    pub npephchemcc: Option<Npephchemcc>,
}

#[derive(Serialize, Deserialize)]
pub struct Ialdnmdkabo {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Npephchemcc {
    #[serde(rename = "HEAVY")]
    Heavy,

    #[serde(rename = "MIDDLE")]
    Middle,
}
