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

pub type ActivityMuqadasPotionMonsterExcelConfigData =
    Vec<ActivityMuqadasPotionMonsterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityMuqadasPotionMonsterExcelConfigDatum {
    #[serde(rename = "describeId")]
    pub describe_id: i64,

    #[serde(rename = "MFILNJFKGJB")]
    pub mfilnjfkgjb: f64,

    #[serde(rename = "NCMFAJNPMFM")]
    pub ncmfajnpmfm: Vec<i64>,

    #[serde(rename = "FBOFPJAADPC")]
    pub fbofpjaadpc: Vec<Fbofpjaadpc>,
}

#[derive(Serialize, Deserialize)]
pub struct Fbofpjaadpc {
    #[serde(rename = "PKMJNJOHGKB")]
    pub pkmjnjohgkb: Vec<f64>,
}
