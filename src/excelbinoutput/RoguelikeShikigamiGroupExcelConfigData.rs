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

pub type RoguelikeShikigamiGroupExcelConfigData = Vec<RoguelikeShikigamiGroupExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RoguelikeShikigamiGroupExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LIKIBHAPBML")]
    pub likibhapbml: Likibhapbml,
}

#[derive(Serialize, Deserialize)]
pub struct Likibhapbml {
    #[serde(rename = "effectType")]
    pub effect_type: String,

    #[serde(rename = "IOGACGMFBKC")]
    pub iogacgmfbkc: String,

    #[serde(rename = "GFPCNPAKAFH")]
    pub gfpcnpakafh: String,
}