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

pub type ActivityVintageDecoExcelConfigData = Vec<ActivityVintageDecoExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityVintageDecoExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "PMFGIHKALJF")]
    pub pmfgihkaljf: Vec<i64>,

    #[serde(rename = "AOEBEJKFEBA")]
    pub aoebejkfeba: Vec<i64>,

    #[serde(rename = "HIONNHIGOCD")]
    pub hionnhigocd: Vec<i64>,

    #[serde(rename = "NLJCBMIJAKM")]
    pub nljcbmijakm: String,

    #[serde(rename = "DFFEMKKBJCC")]
    pub dffemkkbjcc: i64,

    #[serde(rename = "KOHLLLDPFJK")]
    pub kohllldpfjk: i64,

    #[serde(rename = "DOPFPLFNJEG")]
    pub dopfplfnjeg: i64,
}
