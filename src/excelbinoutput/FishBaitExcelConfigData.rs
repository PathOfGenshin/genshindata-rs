// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type FishBaitExcelConfigData = Vec<FishBaitExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FishBaitExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "NIEKDGJFEPA")]
    pub niekdgjfepa: Vec<Niekdgjfepa>,

    #[serde(rename = "BNNKCGELEHJ")]
    pub bnnkcgelehj: i64,

    #[serde(rename = "OPBKGOJOPPG")]
    pub opbkgojoppg: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Niekdgjfepa {
    #[serde(rename = "CCCOBDNGDLN")]
    pub cccobdngdln: Option<i64>,

    #[serde(rename = "weight")]
    pub weight: Option<f64>,

    #[serde(rename = "OLODHCMMKJD")]
    pub olodhcmmkjd: Option<f64>,
}

pub fn load() -> Result<FishBaitExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "FishBaitExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
