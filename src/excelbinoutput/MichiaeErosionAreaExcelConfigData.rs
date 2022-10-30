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

pub type MichiaeErosionAreaExcelConfigData = Vec<MichiaeErosionAreaExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MichiaeErosionAreaExcelConfigDatum {
    #[serde(rename = "FIMKPHPNKOI")]
    pub fimkphpnkoi: Option<i64>,

    #[serde(rename = "IOCMEIPPLDE")]
    pub iocmeipplde: Option<f64>,

    #[serde(rename = "CFGIOOJOIHD")]
    pub cfgioojoihd: Option<f64>,

    #[serde(rename = "INIEFDICDCM")]
    pub iniefdicdcm: Option<f64>,
}
