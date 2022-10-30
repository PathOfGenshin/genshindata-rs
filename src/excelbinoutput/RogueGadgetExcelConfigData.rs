// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type RogueGadgetExcelConfigData = Vec<RogueGadgetExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RogueGadgetExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MKFOLLJINPJ")]
    pub mkfolljinpj: String,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "OFPABOGHPIG")]
    pub ofpaboghpig: Vec<Ofpaboghpig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ofpaboghpig {
    #[serde(rename = "BHBCEHKMHJO")]
    pub bhbcehkmhjo: Option<String>,

    #[serde(rename = "NBNICLENPGL")]
    pub nbniclenpgl: Option<i64>,
}

pub fn load() -> Result<RogueGadgetExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "RogueGadgetExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
