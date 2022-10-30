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

pub type SubSpriteTagExcelConfigData = Vec<SubSpriteTagExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SubSpriteTagExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "FKBEDCPDKIG")]
    pub fkbedcpdkig: String,

    #[serde(rename = "DPICJNGODGH")]
    pub dpicjngodgh: f64,

    #[serde(rename = "AMMJAINBEGN")]
    pub ammjainbegn: Option<f64>,

    #[serde(rename = "PCMFIGFOECB")]
    pub pcmfigfoecb: Option<f64>,
}
