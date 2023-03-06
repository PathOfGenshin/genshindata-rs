// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type SummerTimeV2DungeonStageExcelConfigData = Vec<SummerTimeV2DungeonStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummerTimeV2DungeonStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "prevDungeonId")]
    pub prev_dungeon_id: i64,

    #[serde(rename = "GMCFEEMKPPB")]
    pub gmcfeemkppb: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "questIdList")]
    pub quest_id_list: Vec<i64>,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "MMHAMMOCCKP")]
    pub mmhammocckp: i64,

    #[serde(rename = "BCJBAHPFDJD")]
    pub bcjbahpfdjd: i64,

    #[serde(rename = "JDFEPFFJNCO")]
    pub jdfepffjnco: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "JGDKIIOLODP")]
    pub jgdkiiolodp: i64,

    #[serde(rename = "DLJBKMBKOGD")]
    pub dljbkmbkogd: i64,

    #[serde(rename = "CNMOOFDEOCM")]
    pub cnmoofdeocm: Option<i64>,

    #[serde(rename = "ENIIIBBFGEL")]
    pub eniiibbfgel: Option<i64>,

    #[serde(rename = "AFFLPKNJPJO")]
    pub afflpknjpjo: Vec<i64>,

    #[serde(rename = "GEAEKOMEIBF")]
    pub geaekomeibf: Vec<i64>,

    #[serde(rename = "PMEPEEBAGJC")]
    pub pmepeebagjc: i64,

    #[serde(rename = "ODCAOHPLMPN")]
    pub odcaohplmpn: i64,

    #[serde(rename = "OGLEAMKCEFD")]
    pub ogleamkcefd: i64,

    #[serde(rename = "DILFJILHHOB")]
    pub dilfjilhhob: i64,
}

pub fn load() -> Result<SummerTimeV2DungeonStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "SummerTimeV2DungeonStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
