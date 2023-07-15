/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AranaraCollectionExcelConfigData = Vec<AranaraCollectionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AranaraCollectionExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "PHHOGPAAAPH")]
    pub phhogpaaaph: Phhogpaaaph,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Phhogpaaaph {
    #[serde(rename = "ARANARA_COLLECTION_TYPE_CATALOG_V1")]
    AranaraCollectionTypeCatalogV1,
}
