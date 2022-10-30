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

pub type InvestigationConfigData = Vec<InvestigationConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InvestigationConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "nextInvestigationIdList")]
    pub next_investigation_id_list: Vec<i64>,

    #[serde(rename = "unlockOpenStateType")]
    pub unlock_open_state_type: Option<String>,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "investigationType")]
    pub investigation_type: Option<String>,

    #[serde(rename = "unlockLevel")]
    pub unlock_level: Option<i64>,
}
