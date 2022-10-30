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

pub type CoopCgExcelConfigData = Vec<CoopCgExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CoopCgExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "unlockPointId")]
    pub unlock_point_id: i64,

    #[serde(rename = "cgType")]
    pub cg_type: CgType,

    #[serde(rename = "unlockCond")]
    pub unlock_cond: Vec<UnlockCond>,

    #[serde(rename = "showImageHashSuffix")]
    pub show_image_hash_suffix: i64,

    #[serde(rename = "showImageHashPre")]
    pub show_image_hash_pre: i64,

    #[serde(rename = "showImageSmallHashSuffix")]
    pub show_image_small_hash_suffix: i64,

    #[serde(rename = "showImageSmallHashPre")]
    pub show_image_small_hash_pre: i64,

    #[serde(rename = "cgNameTextMapHash")]
    pub cg_name_text_map_hash: i64,

    #[serde(rename = "cgDesTextMapHash")]
    pub cg_des_text_map_hash: i64,

    #[serde(rename = "sortId")]
    pub sort_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UnlockCond {
    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,

    #[serde(rename = "args")]
    pub args: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum CgType {
    #[serde(rename = "CG_FEMALE")]
    CgFemale,

    #[serde(rename = "CG_MALE")]
    CgMale,
}

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "COOP_COND_COOP_POINT_FINISH")]
    CoopCondCoopPointFinish,
}
