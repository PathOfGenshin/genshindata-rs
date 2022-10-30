// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type HomeWorldNpcExcelConfigData = Vec<HomeWorldNpcExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeWorldNpcExcelConfigDatum {
    #[serde(rename = "furnitureID")]
    pub furniture_id: i64,

    #[serde(rename = "avatarID")]
    pub avatar_id: Option<i64>,

    #[serde(rename = "MECLDFABFMF")]
    pub mecldfabfmf: i64,

    #[serde(rename = "AAIIGOGAJOL")]
    pub aaiigogajol: Vec<i64>,

    #[serde(rename = "AAIAIEHDDHM")]
    pub aaiaiehddhm: Aaiaiehddhm,

    #[serde(rename = "LOIHCKNLOKB")]
    pub loihcknlokb: Loihcknlokb,

    #[serde(rename = "GMPOLDOJIPF")]
    pub gmpoldojipf: Aaiaiehddhm,

    #[serde(rename = "DBGNKJKLOLL")]
    pub dbgnkjkloll: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "PBMEJEABAKN")]
    pub pbmejeabakn: Option<bool>,

    #[serde(rename = "EPLACNGABCG")]
    pub eplacngabcg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Aaiaiehddhm {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_AvatarIcon_Side_Paimon")]
    UiAvatarIconSidePaimon,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Loihcknlokb {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_AvatarIcon_Paimon")]
    UiAvatarIconPaimon,
}

pub fn load() -> Result<HomeWorldNpcExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "HomeWorldNPCExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
