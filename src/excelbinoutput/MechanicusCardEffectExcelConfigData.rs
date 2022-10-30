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

pub type MechanicusCardEffectExcelConfigData = Vec<MechanicusCardEffectExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MechanicusCardEffectExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "targetType")]
    pub target_type: Option<TargetType>,

    #[serde(rename = "targetParamList")]
    pub target_param_list: Vec<i64>,

    #[serde(rename = "effectType")]
    pub effect_type: String,

    #[serde(rename = "effectStrParam")]
    pub effect_str_param: String,

    #[serde(rename = "effectParam1")]
    pub effect_param1: Option<i64>,

    #[serde(rename = "effectParam2")]
    pub effect_param2: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "MECHANICUS_CARD_TARGET_ALL")]
    MechanicusCardTargetAll,

    #[serde(rename = "MECHANICUS_CARD_TARGET_GADGETS")]
    MechanicusCardTargetGadgets,
}
