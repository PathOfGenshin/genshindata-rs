/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type BookSuitExcelConfigData = Vec<BookSuitExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookSuitExcelConfigDatum {
    pub id: i64,
    pub suit_name_text_map_hash: i64,
}
