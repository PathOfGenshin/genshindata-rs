// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type HomeWorldInteractiveNpcExcelConfigData = Vec<Option<serde_json::Value>>;

pub fn load() -> Result<HomeWorldInteractiveNpcExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "HomeWorldInteractiveNPCExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
