// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriFlowerThemeExcelConfigData = Vec<IrodoriFlowerThemeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriFlowerThemeExcelConfigDatum {
    #[serde(rename = "JJCDADOOOEN")]
    pub jjcdadoooen: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "LABNOEGPADP")]
    pub labnoegpadp: i64,

    #[serde(rename = "CIDBMFPGJPE")]
    pub cidbmfpgjpe: i64,

    #[serde(rename = "ABKIJHCOMEO")]
    pub abkijhcomeo: i64,

    #[serde(rename = "PALMPJEIIDN")]
    pub palmpjeiidn: i64,

    #[serde(rename = "EJNPNHDLNOJ")]
    pub ejnpnhdlnoj: i64,

    #[serde(rename = "FOJHAGOCJEB")]
    pub fojhagocjeb: f64,

    #[serde(rename = "HBLFGKHHCAP")]
    pub hblfgkhhcap: Vec<i64>,

    #[serde(rename = "FBNBBAMJJNH")]
    pub fbnbbamjjnh: i64,

    #[serde(rename = "KAOMNLCENAF")]
    pub kaomnlcenaf: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,
}

pub fn load() -> Result<IrodoriFlowerThemeExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "IrodoriFlowerThemeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
