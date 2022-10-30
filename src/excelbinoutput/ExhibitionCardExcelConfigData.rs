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

pub type ExhibitionCardExcelConfigData = Vec<ExhibitionCardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ExhibitionCardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "styleTextMapHash")]
    pub style_text_map_hash: i64,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "KKNFAOCNNPM")]
    pub kknfaocnnpm: Option<Kknfaocnnpm>,
}

#[derive(Serialize, Deserialize)]
pub enum Kknfaocnnpm {
    #[serde(rename = "EXHIBITION_CARD_ORDER_GREATER_BETTER")]
    ExhibitionCardOrderGreaterBetter,
}
