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

pub type RoguelikeCurseExcelConfigData = Vec<RoguelikeCurseExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RoguelikeCurseExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "JNJJOIFBAHP")]
    pub jnjjoifbahp: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "LIKIBHAPBML")]
    pub likibhapbml: Likibhapbml,

    #[serde(rename = "HKHMNJCCKPB")]
    pub hkhmnjcckpb: Option<bool>,

    #[serde(rename = "descParamList")]
    pub desc_param_list: Vec<f64>,

    #[serde(rename = "GBKHEBJBIGC")]
    pub gbkhebjbigc: Vec<bool>,
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
