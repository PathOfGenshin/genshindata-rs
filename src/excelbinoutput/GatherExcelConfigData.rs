// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type GatherExcelConfigData = Vec<GatherExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GatherExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "areaId")]
    pub area_id: Option<i64>,

    #[serde(rename = "pointType")]
    pub point_type: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "extraItemIdVec")]
    pub extra_item_id_vec: Vec<i64>,

    #[serde(rename = "cd")]
    pub cd: i64,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "refreshId")]
    pub refresh_id: Option<i64>,

    #[serde(rename = "blockLimits")]
    pub block_limits: Vec<BlockLimit>,

    #[serde(rename = "initDisableInteract")]
    pub init_disable_interact: Option<bool>,

    #[serde(rename = "FNGDLDEAAPO")]
    pub fngdldeaapo: Option<Fngdldeaapo>,

    #[serde(rename = "pointLocation")]
    pub point_location: Option<PointLocation>,

    #[serde(rename = "isForbidGuest")]
    pub is_forbid_guest: Option<bool>,

    #[serde(rename = "NHEIDFEFCNI")]
    pub nheidfefcni: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockLimit {
    #[serde(rename = "blockId")]
    pub block_id: i64,

    #[serde(rename = "count")]
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Fngdldeaapo {
    #[serde(rename = "GATHER_SAVE_TYPE_HIGH")]
    GatherSaveTypeHigh,

    #[serde(rename = "GATHER_SAVE_TYPE_LOW")]
    GatherSaveTypeLow,

    #[serde(rename = "GATHER_SAVE_TYPE_MID")]
    GatherSaveTypeMid,

    #[serde(rename = "GATHER_SAVE_TYPE_OWN")]
    GatherSaveTypeOwn,
}

#[derive(Serialize, Deserialize)]
pub enum PointLocation {
    #[serde(rename = "POINT_AIR")]
    PointAir,
}
