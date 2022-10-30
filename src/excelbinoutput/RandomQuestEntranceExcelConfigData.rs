// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type RandomQuestEntranceExcelConfigData = Vec<RandomQuestEntranceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterList {
    #[serde(rename = "MHGONGNLHPE")]
    pub mhgongnlhpe: Mhgongnlhpe,

    #[serde(rename = "OACACKFOIAK")]
    pub oacackfoiak: Vec<i64>,

    #[serde(rename = "KCDLJFGFINF")]
    pub kcdljfgfinf: Option<Kcdljfgfinf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Kcdljfgfinf {
    #[serde(rename = "RQ_FILTER_NPC")]
    RqFilterNpc,

    #[serde(rename = "RQ_FILTER_PLAYER_LEVEL")]
    RqFilterPlayerLevel,

    #[serde(rename = "RQ_FILTER_PLAYER_POS_RING")]
    RqFilterPlayerPosRing,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum FilterLogicType {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
}

pub fn load() -> Result<RandomQuestEntranceExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "RandomQuestEntranceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
