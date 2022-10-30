// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityCrystalLinkCondBuffExcelConfigData = Vec<ActivityCrystalLinkCondBuffExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityCrystalLinkCondBuffExcelConfigDatum {
    #[serde(rename = "buffId")]
    pub buff_id: i64,

    #[serde(rename = "GPDAEIMDCBF")]
    pub gpdaeimdcbf: String,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "GPJHNOBPDJI")]
    pub gpjhnobpdji: i64,

    #[serde(rename = "JICNBGIOOHI")]
    pub jicnbgioohi: i64,

    #[serde(rename = "NBLOCEJHFCN")]
    pub nblocejhfcn: i64,

    #[serde(rename = "HPIKALMBHLL")]
    pub hpikalmbhll: i64,

    #[serde(rename = "FFDCLDPEEML")]
    pub ffdcldpeeml: Vec<String>,
}

pub fn load() -> Result<ActivityCrystalLinkCondBuffExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityCrystalLinkCondBuffExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
