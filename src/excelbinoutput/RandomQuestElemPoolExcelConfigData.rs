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

pub type RandomQuestElemPoolExcelConfigData = Vec<RandomQuestElemPoolExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RandomQuestElemPoolExcelConfigDatum {
    #[serde(rename = "_id")]
    pub id: i64,

    #[serde(rename = "_poolId")]
    pub pool_id: i64,

    #[serde(rename = "_weight")]
    pub weight: i64,

    #[serde(rename = "_sampleList")]
    pub sample_list: Vec<SampleList>,
}

#[derive(Serialize, Deserialize)]
pub struct SampleList {
    #[serde(rename = "_type")]
    pub sample_list_type: Type,

    #[serde(rename = "_content")]
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "{GivingCondID1}")]
    GivingCondId1,

    #[serde(rename = "{GivingCondID2}")]
    GivingCondId2,

    #[serde(rename = "{GivingCondID3}")]
    GivingCondId3,

    #[serde(rename = "{GivingID}")]
    GivingId,

    #[serde(rename = "{MonsterLevel}")]
    MonsterLevel,

    #[serde(rename = "{NPCPos}")]
    NpcPos,

    #[serde(rename = "{PatternID}")]
    PatternId,

    #[serde(rename = "{PhotoID}")]
    PhotoId,

    #[serde(rename = "{PlayerLevel}")]
    PlayerLevel,

    #[serde(rename = "{QuestGatherID}")]
    QuestGatherId,

    #[serde(rename = "{QuestGatherID2}")]
    QuestGatherId2,

    #[serde(rename = "{QuestGatherNum}")]
    QuestGatherNum,

    #[serde(rename = "{QuestGlabalVarID}")]
    QuestGlabalVarId,

    #[serde(rename = "{QuestNpcID}")]
    QuestNpcId,

    #[serde(rename = "{QuestNpcID2}")]
    QuestNpcId2,

    #[serde(rename = "{QuestNpcID3}")]
    QuestNpcId3,

    #[serde(rename = "{QuestRescuePos}")]
    QuestRescuePos,

    #[serde(rename = "{QuestRescuePos1}")]
    QuestRescuePos1,

    #[serde(rename = "{QuestRescuePos2}")]
    QuestRescuePos2,
}
