// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MichiaeErosionAreaExcelConfigData = Vec<MichiaeErosionAreaExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MichiaeErosionAreaExcelConfigDatum {
    #[serde(rename = "FIMKPHPNKOI")]
    pub fimkphpnkoi: Option<i64>,

    #[serde(rename = "IOCMEIPPLDE")]
    pub iocmeipplde: Option<f64>,

    #[serde(rename = "CFGIOOJOIHD")]
    pub cfgioojoihd: Option<f64>,

    #[serde(rename = "INIEFDICDCM")]
    pub iniefdicdcm: Option<f64>,
}

pub fn load() -> Result<MichiaeErosionAreaExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MichiaeErosionAreaExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
