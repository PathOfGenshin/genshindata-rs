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

pub type ExploreAreaTotalExpExcelConfigData = Vec<ExploreAreaTotalExpExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ExploreAreaTotalExpExcelConfigDatum {
    #[serde(rename = "areaID")]
    pub area_id: i64,

    #[serde(rename = "totalExp")]
    pub total_exp: i64,

    #[serde(rename = "BAJLLPANAAP")]
    pub bajllpanaap: Option<f64>,
}
