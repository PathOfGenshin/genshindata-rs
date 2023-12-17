/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FishblasterPieceDataData = Vec<FishblasterPieceDataDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FishblasterPieceDataDatum {
    pub id: i64,
    pub name_text_map_hash: i64,
    pub desc_text_map_hash: i64,
    #[serde(rename = "CFCBFBNEENM")]
    pub cfcbfbneenm: i64,
    pub icon: String,
    #[serde(rename = "HDIEIAGNMAF")]
    pub hdieiagnmaf: String,
    #[serde(rename = "type")]
    pub fishblaster_piece_data_datum_type: Option<i64>,
}
