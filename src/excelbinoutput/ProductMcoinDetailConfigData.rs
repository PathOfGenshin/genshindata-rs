// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ProductMcoinDetailConfigData = Vec<ProductMcoinDetailConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductMcoinDetailConfigDatum {
    #[serde(rename = "itemNameTextMapHash")]
    pub item_name_text_map_hash: i64,

    #[serde(rename = "primNameTextMapHash")]
    pub prim_name_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "mcoinNum")]
    pub mcoin_num: i64,

    #[serde(rename = "mcoinFirst")]
    pub mcoin_first: i64,

    #[serde(rename = "seqence")]
    pub seqence: i64,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "priceTier")]
    pub price_tier: String,

    #[serde(rename = "shopType")]
    pub shop_type: String,

    #[serde(rename = "mcoinNonFirst")]
    pub mcoin_non_first: Option<i64>,
}

pub fn load() -> Result<ProductMcoinDetailConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ProductMcoinDetailConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
