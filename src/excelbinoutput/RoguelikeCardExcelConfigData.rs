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

pub type RoguelikeCardExcelConfigData = Vec<RoguelikeCardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RoguelikeCardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sortOrder")]
    pub sort_order: i64,

    #[serde(rename = "type")]
    pub roguelike_card_excel_config_datum_type: Type,

    #[serde(rename = "FALACHEECPE")]
    pub falacheecpe: Vec<i64>,

    #[serde(rename = "JDELHOAAIJH")]
    pub jdelhoaaijh: Vec<String>,

    #[serde(rename = "LIKIBHAPBML")]
    pub likibhapbml: Likibhapbml,

    #[serde(rename = "FFMGPDNCIEI")]
    pub ffmgpdnciei: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "JBLCGBFOOLF")]
    pub jblcgbfoolf: i64,

    #[serde(rename = "descParamList")]
    pub desc_param_list: Vec<f64>,

    #[serde(rename = "GBKHEBJBIGC")]
    pub gbkhebjbigc: Vec<bool>,

    #[serde(rename = "CEPPLGFOAHB")]
    pub cepplgfoahb: Vec<f64>,

    #[serde(rename = "JAOLFFLPFEJ")]
    pub jaolfflpfej: Option<Jaolfflpfej>,

    #[serde(rename = "HKHMNJCCKPB")]
    pub hkhmnjcckpb: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Likibhapbml {
    #[serde(rename = "effectType")]
    pub effect_type: String,

    #[serde(rename = "IOGACGMFBKC")]
    pub iogacgmfbkc: String,

    #[serde(rename = "GFPCNPAKAFH")]
    pub gfpcnpakafh: String,

    #[serde(rename = "DJLMHFODHDL")]
    pub djlmhfodhdl: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Jaolfflpfej {
    #[serde(rename = "ROGUELIKE_CARD_LABEL_EQUIPMENT")]
    RoguelikeCardLabelEquipment,

    #[serde(rename = "ROGUELIKE_CARD_LABEL_LEVEL")]
    RoguelikeCardLabelLevel,

    #[serde(rename = "ROGUELIKE_CARD_LABEL_RUNE")]
    RoguelikeCardLabelRune,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ROGUELIKE_CARD_TPYE_R")]
    RoguelikeCardTpyeR,

    #[serde(rename = "ROGUELIKE_CARD_TPYE_SR")]
    RoguelikeCardTpyeSr,

    #[serde(rename = "ROGUELIKE_CARD_TPYE_SSR")]
    RoguelikeCardTpyeSsr,
}
