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

pub type RandomQuestEntranceExcelConfigData = Vec<RandomQuestEntranceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RandomQuestEntranceExcelConfigDatum {
    #[serde(rename = "_id")]
    pub id: i64,

    #[serde(rename = "_weight")]
    pub weight: i64,

    #[serde(rename = "_templateId")]
    pub template_id: i64,

    #[serde(rename = "_filterList")]
    pub filter_list: Vec<FilterList>,

    #[serde(rename = "_filterLogicType")]
    pub filter_logic_type: Option<FilterLogicType>,
}

#[derive(Serialize, Deserialize)]
pub struct FilterList {
    #[serde(rename = "MHGONGNLHPE")]
    pub mhgongnlhpe: Mhgongnlhpe,

    #[serde(rename = "OACACKFOIAK")]
    pub oacackfoiak: Vec<i64>,

    #[serde(rename = "KCDLJFGFINF")]
    pub kcdljfgfinf: Option<Kcdljfgfinf>,
}

#[derive(Serialize, Deserialize)]
pub enum Kcdljfgfinf {
    #[serde(rename = "RQ_FILTER_NPC")]
    RqFilterNpc,

    #[serde(rename = "RQ_FILTER_PLAYER_LEVEL")]
    RqFilterPlayerLevel,

    #[serde(rename = "RQ_FILTER_PLAYER_POS_RING")]
    RqFilterPlayerPosRing,
}

#[derive(Serialize, Deserialize)]
pub enum Mhgongnlhpe {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "{PlayerLevel}")]
    PlayerLevel,

    #[serde(rename = "{QuestNpcID}")]
    QuestNpcId,

    #[serde(rename = "{QuestNpcID2}")]
    QuestNpcId2,

    #[serde(rename = "{QuestRescuePos}")]
    QuestRescuePos,

    #[serde(rename = "{QuestRescuePos1}")]
    QuestRescuePos1,
}

#[derive(Serialize, Deserialize)]
pub enum FilterLogicType {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
}
