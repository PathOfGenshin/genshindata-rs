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

pub type H5ActivityWatcherExcelConfigData = Vec<H5ActivityWatcherExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct H5ActivityWatcherExcelConfigDatum {
    #[serde(rename = "h5ActivityId")]
    pub h5_activity_id: i64,

    #[serde(rename = "condComb")]
    pub cond_comb: CondComb,

    #[serde(rename = "condList")]
    pub cond_list: Vec<CondList>,

    #[serde(rename = "isDailyRefresh")]
    pub is_daily_refresh: Option<bool>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CondList {
    #[serde(rename = "type")]
    pub cond_list_type: Option<Type>,

    #[serde(rename = "paramStr")]
    pub param_str: String,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum CondComb {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "H5_ACTIVITY_COND_DAY_COUNT_GREAT_EQUAL")]
    H5ActivityCondDayCountGreatEqual,
}
