/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityGearGadgetJigsawExcelConfigData = Vec<ActivityGearGadgetJigsawExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityGearGadgetJigsawExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "gadgetID")]
    pub gadget_id: i64,
    #[serde(rename = "MMDFBEKNPDA")]
    pub mmdfbeknpda: i64,
    #[serde(rename = "materialID")]
    pub material_id: i64,
    #[serde(rename = "MFNLEGLFGAP")]
    pub mfnleglfgap: String,
}
