// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type VintageMarketStoreExcelConfigData = Vec<VintageMarketStoreExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketStoreExcelConfigDatum {
    #[serde(rename = "MEPDABCIKFD")]
    pub mepdabcikfd: i64,

    #[serde(rename = "MAHDCLCKDJI")]
    pub mahdclckdji: i64,

    #[serde(rename = "NPEKDLNHCPG")]
    pub npekdlnhcpg: i64,

    #[serde(rename = "IPJCFCNJKKI")]
    pub ipjcfcnjkki: Vec<i64>,

    #[serde(rename = "AJLAEJBALCK")]
    pub ajlaejbalck: Vec<Ajlaejbalck>,

    #[serde(rename = "FNPECGOHLDP")]
    pub fnpecgohldp: i64,

    #[serde(rename = "NMNJDBJDCIB")]
    pub nmnjdbjdcib: Vec<i64>,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "PBMGLKBOKEB")]
    pub pbmglkbokeb: i64,

    #[serde(rename = "DNDHGOIPAEE")]
    pub dndhgoipaee: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ajlaejbalck {
    #[serde(rename = "defaultValue")]
    pub default_value: i64,

    #[serde(rename = "LFOLKMHENDI")]
    pub lfolkmhendi: i64,
}

pub fn load() -> Result<VintageMarketStoreExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "VintageMarketStoreExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
