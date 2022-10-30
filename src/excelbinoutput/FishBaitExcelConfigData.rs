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

pub type FishBaitExcelConfigData = Vec<FishBaitExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FishBaitExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "NIEKDGJFEPA")]
    pub niekdgjfepa: Vec<Niekdgjfepa>,

    #[serde(rename = "BNNKCGELEHJ")]
    pub bnnkcgelehj: i64,

    #[serde(rename = "OPBKGOJOPPG")]
    pub opbkgojoppg: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Niekdgjfepa {
    #[serde(rename = "CCCOBDNGDLN")]
    pub cccobdngdln: Option<i64>,

    #[serde(rename = "weight")]
    pub weight: Option<f64>,

    #[serde(rename = "OLODHCMMKJD")]
    pub olodhcmmkjd: Option<f64>,
}
