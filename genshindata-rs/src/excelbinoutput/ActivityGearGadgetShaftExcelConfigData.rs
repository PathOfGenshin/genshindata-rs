// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityGearGadgetShaftExcelConfigData = Vec<ActivityGearGadgetShaftExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityGearGadgetShaftExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "gadgetID")]
    pub gadget_id: i64,

    #[serde(rename = "radius")]
    pub radius: f64,

    #[serde(rename = "EFNJMMNFBGI")]
    pub efnjmmnfbgi: Vec<f64>,

    #[serde(rename = "MCPCNKCOJBO")]
    pub mcpcnkcojbo: f64,

    #[serde(rename = "GCNLKELCKCI")]
    pub gcnlkelckci: Vec<f64>,
}

pub fn load() -> Result<ActivityGearGadgetShaftExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityGearGadgetShaftExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}