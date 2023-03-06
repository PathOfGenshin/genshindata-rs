// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ProductIdConfigData = Vec<ProductIdConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductIdConfigDatum {
    #[serde(rename = "productId")]
    pub product_id: String,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "isInternal")]
    pub is_internal: Option<bool>,

    #[serde(rename = "entitlementId")]
    pub entitlement_id: String,

    #[serde(rename = "HFBCHMMGGHI")]
    pub hfbchmmgghi: String,

    #[serde(rename = "KAHONNGIIAC")]
    pub kahonngiiac: Vec<Kahonngiiac>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Kahonngiiac {
    #[serde(rename = "CLOUD_ANDROID")]
    CloudAndroid,

    #[serde(rename = "CLOUD_IOS")]
    CloudIos,

    #[serde(rename = "CLOUD_MAC")]
    CloudMac,

    #[serde(rename = "CLOUD_PC")]
    CloudPc,
}

pub fn load() -> Result<ProductIdConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ProductIdConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
