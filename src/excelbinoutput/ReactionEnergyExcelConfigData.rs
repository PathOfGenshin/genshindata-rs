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

pub type ReactionEnergyExcelConfigData = Vec<ReactionEnergyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReactionEnergyExcelConfigDatum {
    #[serde(rename = "type")]
    pub reaction_energy_excel_config_datum_type: String,

    #[serde(rename = "minDurability")]
    pub min_durability: f64,

    #[serde(rename = "maxDurability")]
    pub max_durability: f64,

    #[serde(rename = "costRatio")]
    pub cost_ratio: f64,

    #[serde(rename = "configID")]
    pub config_id: i64,

    #[serde(rename = "poolSize")]
    pub pool_size: f64,

    #[serde(rename = "poolRevivePeriod")]
    pub pool_revive_period: f64,

    #[serde(rename = "poolReviveEnergy")]
    pub pool_revive_energy: f64,

    #[serde(rename = "isPersistent")]
    pub is_persistent: Option<bool>,

    #[serde(rename = "costPeriod")]
    pub cost_period: Option<f64>,

    #[serde(rename = "dropProb")]
    pub drop_prob: i64,
}
