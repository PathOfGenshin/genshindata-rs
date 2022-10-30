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

pub type CustomGadgetSlotLevelTagConfigData = Vec<CustomGadgetSlotLevelTagConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CustomGadgetSlotLevelTagConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "MAEPKPDBDLP")]
    pub maepkpdbdlp: String,

    #[serde(rename = "HOPKJJEIAGH")]
    pub hopkjjeiagh: Vec<Hopkjjeiagh>,

    #[serde(rename = "IHAAKMMPFNM")]
    pub ihaakmmpfnm: i64,

    #[serde(rename = "MDAJLNBOPMB")]
    pub mdajlnbopmb: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Hopkjjeiagh {
    #[serde(rename = "NBIIFLDGLGM")]
    pub nbiifldglgm: Nbiifldglgm,

    #[serde(rename = "PHEDCEGDALJ")]
    pub phedcegdalj: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Nbiifldglgm {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "LeftHill")]
    LeftHill,

    #[serde(rename = "RightHill")]
    RightHill,
}
