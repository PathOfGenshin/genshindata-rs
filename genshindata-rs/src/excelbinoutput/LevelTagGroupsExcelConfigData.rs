// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type LevelTagGroupsExcelConfigData = Vec<LevelTagGroupsExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelTagGroupsExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "LNDNJDKLPCA")]
    pub lndnjdklpca: Vec<Lndnjdklpca>,

    #[serde(rename = "DPIHKLDDOGC")]
    pub dpihklddogc: Vec<i64>,

    #[serde(rename = "DNEOKPJIAMI")]
    pub dneokpjiami: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lndnjdklpca {
    #[serde(rename = "JCAMMOFCBON")]
    pub jcammofcbon: Vec<i64>,
}

pub fn load() -> Result<LevelTagGroupsExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "LevelTagGroupsExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}