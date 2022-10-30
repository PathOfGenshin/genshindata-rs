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

pub type MechanicusGearLevelUpExcelConfigData = Vec<MechanicusGearLevelUpExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MechanicusGearLevelUpExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "gearID")]
    pub gear_id: i64,

    #[serde(rename = "gearLevel")]
    pub gear_level: i64,

    #[serde(rename = "gearNameTextMapHash")]
    pub gear_name_text_map_hash: i64,

    #[serde(rename = "gearShortNameTextMapHash")]
    pub gear_short_name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "gearIconPath")]
    pub gear_icon_path: String,

    #[serde(rename = "attack")]
    pub attack: Option<i64>,

    #[serde(rename = "attackSpeed")]
    pub attack_speed: Option<i64>,

    #[serde(rename = "attackRange")]
    pub attack_range: Option<i64>,

    #[serde(rename = "buildCost")]
    pub build_cost: i64,

    #[serde(rename = "demolitionRefund")]
    pub demolition_refund: i64,

    #[serde(rename = "globalValueParam")]
    pub global_value_param: Vec<GlobalValueParam>,

    #[serde(rename = "effectList")]
    pub effect_list: Vec<i64>,

    #[serde(rename = "gearLevelUpMoney")]
    pub gear_level_up_money: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalValueParam {
    #[serde(rename = "key")]
    pub key: Key,

    #[serde(rename = "value")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "SGV_TDLevel")]
    SgvTdLevel,
}
