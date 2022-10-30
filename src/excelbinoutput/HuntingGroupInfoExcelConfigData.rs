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

pub type HuntingGroupInfoExcelConfigData = Vec<HuntingGroupInfoExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HuntingGroupInfoExcelConfigDatum {
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "regionId")]
    pub region_id: i64,

    #[serde(rename = "pointType")]
    pub point_type: Option<PointType>,

    #[serde(rename = "refIndex")]
    pub ref_index: Vec<i64>,

    #[serde(rename = "posType")]
    pub pos_type: Option<PosType>,
}

#[derive(Serialize, Deserialize)]
pub enum PointType {
    #[serde(rename = "HUNTING_CLUE_FINAL")]
    HuntingClueFinal,

    #[serde(rename = "HUNTING_CLUE_GATHER")]
    HuntingClueGather,

    #[serde(rename = "HUNTING_CLUE_MONSTER")]
    HuntingClueMonster,
}

#[derive(Serialize, Deserialize)]
pub enum PosType {
    #[serde(rename = "HUNTING_POS_GROUND")]
    HuntingPosGround,

    #[serde(rename = "HUNTING_POS_SHOAL_WATER")]
    HuntingPosShoalWater,
}
