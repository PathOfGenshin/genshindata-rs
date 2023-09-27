/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type CatalogExcelConfigData = Vec<CatalogExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "type")]
    pub catalog_excel_config_datum_type: String,
    pub tab_list: Vec<Vec<i64>>,
    #[serde(rename = "FKOLBFIBCFJ")]
    pub fkolbfibcfj: Vec<HashMap<String, i64>>,
    #[serde(rename = "KGKFFOLKLDD")]
    pub kgkffolkldd: Vec<i64>,
}
