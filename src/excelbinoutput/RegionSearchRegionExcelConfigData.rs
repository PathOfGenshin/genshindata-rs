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

pub type RegionSearchRegionExcelConfigData = Vec<RegionSearchRegionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RegionSearchRegionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "regionRadius")]
    pub region_radius: f64,

    #[serde(rename = "oneoffGroupList")]
    pub oneoff_group_list: Vec<i64>,

    #[serde(rename = "oneoffGroupNum")]
    pub oneoff_group_num: i64,

    #[serde(rename = "recycleGroupList")]
    pub recycle_group_list: Vec<i64>,

    #[serde(rename = "recycleGroupNum")]
    pub recycle_group_num: i64,

    #[serde(rename = "recycleType")]
    pub recycle_type: String,

    #[serde(rename = "recycleParam")]
    pub recycle_param: i64,
}
