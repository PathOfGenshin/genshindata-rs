/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcGadgetAffectRangExcelConfigData = Vec<UgcGadgetAffectRangExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UgcGadgetAffectRangExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "OHHHICFMOKP")]
    pub ohhhicfmokp: String,
}
