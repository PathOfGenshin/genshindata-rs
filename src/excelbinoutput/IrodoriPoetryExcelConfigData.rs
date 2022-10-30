// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type IrodoriPoetryExcelConfigData = Vec<IrodoriPoetryExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriPoetryExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "LHICMKKPEOO")]
    pub lhicmkkpeoo: i64,

    #[serde(rename = "HAFOEGJLOAK")]
    pub hafoegjloak: i64,

    #[serde(rename = "entityType")]
    pub entity_type: String,

    #[serde(rename = "CMECOBJCEEG")]
    pub cmecobjceeg: Vec<Cmecobjceeg>,

    #[serde(rename = "MPJOBAMPMPP")]
    pub mpjobampmpp: i64,

    #[serde(rename = "BAPMODDGGIP")]
    pub bapmoddggip: i64,

    #[serde(rename = "CCDMHAGGCCK")]
    pub ccdmhaggcck: i64,

    #[serde(rename = "EGBHFDJBCAB")]
    pub egbhfdjbcab: Vec<i64>,

    #[serde(rename = "watcherID")]
    pub watcher_id: i64,

    #[serde(rename = "PLMJBCDJHHA")]
    pub plmjbcdjhha: i64,

    #[serde(rename = "JILNEOHCOHP")]
    pub jilneohcohp: i64,

    #[serde(rename = "JMOEDAINFKE")]
    pub jmoedainfke: i64,

    #[serde(rename = "LGDJFFDJGCL")]
    pub lgdjffdjgcl: i64,

    #[serde(rename = "KHEDKNOOMDC")]
    pub khedknoomdc: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cmecobjceeg {
    #[serde(rename = "AEKMNCGECNO")]
    pub aekmncgecno: Vec<i64>,

    #[serde(rename = "EPEDECCNOLI")]
    pub epedeccnoli: Option<i64>,
}

pub fn load() -> Result<IrodoriPoetryExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "IrodoriPoetryExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
