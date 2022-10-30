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

pub type PersonalLineActivityExcelConfigData = Vec<PersonalLineActivityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PersonalLineActivityExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "perfabPath")]
    pub perfab_path: String,

    #[serde(rename = "OJOGGBMJOMI")]
    pub ojoggbmjomi: Option<String>,
}
