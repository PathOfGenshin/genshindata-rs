/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HomeWorldFarmFieldExcelConfigData = Vec<HomeWorldFarmFieldExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeWorldFarmFieldExcelConfigDatum {
    #[serde(rename = "fieldItemID")]
    pub field_item_id: i64,
    pub field_type: String,
    #[serde(rename = "fieldGadgetID")]
    pub field_gadget_id: i64,
    pub field_slot_num: i64,
    #[serde(rename = "fieldSlotGadgetID")]
    pub field_slot_gadget_id: Vec<i64>,
}
