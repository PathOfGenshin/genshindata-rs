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

pub type OpenStateConfigData = Vec<OpenStateConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OpenStateConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "allowClientOpen")]
    pub allow_client_open: Option<bool>,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,

    #[serde(rename = "defaultState")]
    pub default_state: Option<bool>,

    #[serde(rename = "systemOpenUiId")]
    pub system_open_ui_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,

    #[serde(rename = "param")]
    pub param: Option<i64>,

    #[serde(rename = "param2")]
    pub param2: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "OPEN_STATE_CITY_REPUTATION_LEVEL")]
    OpenStateCityReputationLevel,

    #[serde(rename = "OPEN_STATE_COND_PARENT_QUEST")]
    OpenStateCondParentQuest,

    #[serde(rename = "OPEN_STATE_COND_PLAYER_LEVEL")]
    OpenStateCondPlayerLevel,

    #[serde(rename = "OPEN_STATE_COND_QUEST")]
    OpenStateCondQuest,

    #[serde(rename = "OPEN_STATE_OFFERING_LEVEL")]
    OpenStateOfferingLevel,
}
