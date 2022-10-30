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

pub type BartenderEventExcelConfigData = Vec<BartenderEventExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BartenderEventExcelConfigDatum {
    #[serde(rename = "effectName")]
    pub effect_name: String,

    #[serde(rename = "effectType")]
    pub effect_type: Option<EffectType>,

    #[serde(rename = "AHOLKAOOFMD")]
    pub aholkaoofmd: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum EffectType {
    #[serde(rename = "BARTENDER_FINISH_CUP_EFF")]
    BartenderFinishCupEff,

    #[serde(rename = "BARTENDER_FINISH_CUP_ONHAND")]
    BartenderFinishCupOnhand,

    #[serde(rename = "BARTENDER_FINISH_CUP_ONTABLE")]
    BartenderFinishCupOntable,

    #[serde(rename = "BARTENDER_FINISH_CUP_OVER")]
    BartenderFinishCupOver,

    #[serde(rename = "BARTENDER_FINISH_CUP_PUSH")]
    BartenderFinishCupPush,

    #[serde(rename = "BARTENDER_FINISH_POUR_EFF")]
    BartenderFinishPourEff,

    #[serde(rename = "BARTENDER_INGREDIENTS")]
    BartenderIngredients,

    #[serde(rename = "BARTENDER_INGREDIENTS_POUR_EFF")]
    BartenderIngredientsPourEff,

    #[serde(rename = "BARTENDER_MIX_BAR")]
    BartenderMixBar,

    #[serde(rename = "BARTENDER_ORIGIN_CUP_ONHAND")]
    BartenderOriginCupOnhand,
}
