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

pub type BlessingScanExcelConfigData = Vec<BlessingScanExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BlessingScanExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "typeId")]
    pub type_id: i64,

    #[serde(rename = "scanType")]
    pub scan_type: ScanType,

    #[serde(rename = "refId")]
    pub ref_id: i64,

    #[serde(rename = "picUpConfig")]
    pub pic_up_config: Vec<PicUpConfig>,

    #[serde(rename = "scanTime")]
    pub scan_time: i64,

    #[serde(rename = "hitBoxesNodeName")]
    pub hit_boxes_node_name: HitBoxesNodeName,
}

#[derive(Serialize, Deserialize)]
pub struct PicUpConfig {}

#[derive(Serialize, Deserialize)]
pub enum HitBoxesNodeName {
    #[serde(rename = "BodyBox")]
    BodyBox,

    #[serde(rename = "")]
    Empty,

    #[serde(rename = "HitBox")]
    HitBox,

    #[serde(rename = "RootNode")]
    RootNode,
}

#[derive(Serialize, Deserialize)]
pub enum ScanType {
    #[serde(rename = "BLESSING_SCAN_TYPE_GATHER")]
    BlessingScanTypeGather,

    #[serde(rename = "BLESSING_SCAN_TYPE_MONSTER")]
    BlessingScanTypeMonster,
}
