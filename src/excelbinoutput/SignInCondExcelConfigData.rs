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

pub type SignInCondExcelConfigData = Vec<SignInCondExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SignInCondExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "condList")]
    pub cond_list: Vec<CondList>,

    #[serde(rename = "totalDayCount")]
    pub total_day_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CondList {
    #[serde(rename = "type")]
    pub cond_list_type: Option<Type>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SIGN_IN_COND_PLAYER_LEVEL")]
    SignInCondPlayerLevel,
}
