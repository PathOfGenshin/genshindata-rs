// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type BartenderTaskOrderExcelConfigData = Vec<BartenderTaskOrderExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BartenderTaskOrderExcelConfigDatum {
    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "KGNOONGAMBK")]
    pub kgnoongambk: Vec<i64>,

    #[serde(rename = "CLHIEOCLBLH")]
    pub clhieoclblh: i64,

    #[serde(rename = "NHIOPIHEEEC")]
    pub nhiopiheeec: i64,

    #[serde(rename = "OKIABFOPABC")]
    pub okiabfopabc: i64,

    #[serde(rename = "HKAHLGLDJFJ")]
    pub hkahlgldjfj: Option<i64>,

    #[serde(rename = "LHHCCOGPFPN")]
    pub lhhccogpfpn: Option<bool>,
}

pub fn load() -> Result<BartenderTaskOrderExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "BartenderTaskOrderExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}