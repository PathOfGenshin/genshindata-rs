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

pub type ActivitySumoDifficultyExcelConfigData = Vec<ActivitySumoDifficultyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySumoDifficultyExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "BOMLNJNENPD")]
    pub bomlnjnenpd: i64,

    #[serde(rename = "DKHKDONKPOF")]
    pub dkhkdonkpof: i64,

    #[serde(rename = "IAPAGNFKHOE")]
    pub iapagnfkhoe: f64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "difficulty")]
    pub difficulty: Option<String>,
}
