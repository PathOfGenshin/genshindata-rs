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

pub type OfferingLevelUpExcelConfigData = Vec<OfferingLevelUpExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OfferingLevelUpExcelConfigDatum {
    #[serde(rename = "offeringId")]
    pub offering_id: i64,

    #[serde(rename = "level")]
    pub level: Option<i64>,

    #[serde(rename = "actionVec")]
    pub action_vec: Vec<ActionVec>,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,

    #[serde(rename = "NBGDLGGGIDN")]
    pub nbgdlgggidn: Vec<Nbgdlgggidn>,

    #[serde(rename = "cutSceneId")]
    pub cut_scene_id: Option<i64>,

    #[serde(rename = "consumeItemConfig")]
    pub consume_item_config: Option<bool>,

    #[serde(rename = "BGKGEMDBAIA")]
    pub bgkgemdbaia: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ActionVec {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Nbgdlgggidn {
    #[serde(rename = "param")]
    pub param: String,

    #[serde(rename = "actionType")]
    pub action_type: Option<ActionType>,
}

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "OFFERING_ACTION_CLOSE_ROUTINE")]
    OfferingActionCloseRoutine,

    #[serde(rename = "OFFERING_ACTION_NOTIFY_GROUP")]
    OfferingActionNotifyGroup,

    #[serde(rename = "OFFERING_ACTION_OPEN_ROUTINE")]
    OfferingActionOpenRoutine,

    #[serde(rename = "OFFERING_ACTION_SET_GADGET_CHAIN_LEVEL")]
    OfferingActionSetGadgetChainLevel,
}
