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

pub type NightCrowArgumentExcelConfigData = Vec<NightCrowArgumentExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NightCrowArgumentExcelConfigDatum {
    #[serde(rename = "MPPIJDJGHKL")]
    pub mppijdjghkl: i64,

    #[serde(rename = "MIMGLOJIIKC")]
    pub mimglojiikc: Vec<i64>,

    #[serde(rename = "DIDKFCBLLHB")]
    pub didkfcbllhb: Didkfcbllhb,

    #[serde(rename = "OHNFCLJNPOF")]
    pub ohnfcljnpof: String,

    #[serde(rename = "BIFNFDOHLLB")]
    pub bifnfdohllb: String,

    #[serde(rename = "EOFIGGACJAM")]
    pub eofiggacjam: String,

    #[serde(rename = "EDFEONLPHKA")]
    pub edfeonlphka: String,

    #[serde(rename = "BPOKOEPCDNN")]
    pub bpokoepcdnn: Bpokoepcdnn,

    #[serde(rename = "OHENDBMOPLL")]
    pub ohendbmopll: Ohendbmopll,

    #[serde(rename = "PFMOPCAANNI")]
    pub pfmopcaanni: String,
}

#[derive(Serialize, Deserialize)]
pub enum Bpokoepcdnn {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "-1,1,-0.01,1,-2,2")]
    The11001122,

    #[serde(rename = "-1,1,-0.457,2,-2,2.6")]
    The1104572226,
}

#[derive(Serialize, Deserialize)]
pub enum Didkfcbllhb {
    #[serde(rename = "0,1.5,0")]
    The0150,
}

#[derive(Serialize, Deserialize)]
pub enum Ohendbmopll {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "-14.89,86.69,160.33")]
    The1489866916033,

    #[serde(rename = "28.44794,43,-10.34")]
    The2844794431034,
}
