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

pub type GadgetInteractExcelConfigData = Vec<GadgetInteractExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GadgetInteractExcelConfigDatum {
    #[serde(rename = "interactId")]
    pub interact_id: i64,

    #[serde(rename = "actionType")]
    pub action_type: Option<ActionType>,

    #[serde(rename = "param1")]
    pub param1: Option<i64>,

    #[serde(rename = "LCGFBNBDOCN")]
    pub lcgfbnbdocn: Vec<Lcgfbnbdocn>,

    #[serde(rename = "costItems")]
    pub cost_items: Vec<CostItem>,

    #[serde(rename = "uiTitleTextMapHash")]
    pub ui_title_text_map_hash: i64,

    #[serde(rename = "uiDescTextMapHash")]
    pub ui_desc_text_map_hash: i64,

    #[serde(rename = "condList")]
    pub cond_list: Vec<CondList>,

    #[serde(rename = "consumeItemNum")]
    pub consume_item_num: Option<bool>,

    #[serde(rename = "consumeItemId")]
    pub consume_item_id: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct CondList {
    #[serde(rename = "param")]
    pub param: Vec<String>,

    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,
}

#[derive(Serialize, Deserialize)]
pub struct CostItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Lcgfbnbdocn {
    #[serde(rename = "actionType")]
    pub action_type: Option<ActionType>,

    #[serde(rename = "param")]
    pub param: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "INTERACT_ACTION_CONSUME_REGIONAL_PLAY_VAR")]
    InteractActionConsumeRegionalPlayVar,

    #[serde(rename = "INTERACT_ACTION_SET_GADGET_CHAIN_BUFF")]
    InteractActionSetGadgetChainBuff,

    #[serde(rename = "INTERACT_ACTION_STATE")]
    InteractActionState,

    #[serde(rename = "INTERACT_ACTION_UNLOCK_SPECIAL_TRANSPORT_POINT")]
    InteractActionUnlockSpecialTransportPoint,
}

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "INTERACT_COND_OFFERING_LEVEL")]
    InteractCondOfferingLevel,

    #[serde(rename = "INTERACT_COND_REGIONAL_PLAY_VAR_GREATER_THAN")]
    InteractCondRegionalPlayVarGreaterThan,

    #[serde(rename = "INTERACT_COND_WIDGET_ON")]
    InteractCondWidgetOn,
}
