// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type VehicleSkillDepotExcelConfigData = Vec<VehicleSkillDepotExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleSkillDepotExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "CLMLAJFIBEF")]
    pub clmlajfibef: Vec<i64>,
}

pub fn load() -> Result<VehicleSkillDepotExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "VehicleSkillDepotExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
