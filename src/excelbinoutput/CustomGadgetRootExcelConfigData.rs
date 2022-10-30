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

pub type CustomGadgetRootExcelConfigData = Vec<CustomGadgetRootExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CustomGadgetRootExcelConfigDatum {
    #[serde(rename = "NLMJCEJOGKK")]
    pub nlmjcejogkk: i64,

    #[serde(rename = "MAEPKPDBDLP")]
    pub maepkpdbdlp: String,

    #[serde(rename = "KBJGAEKFCGL")]
    pub kbjgaekfcgl: Kbjgaekfcgl,

    #[serde(rename = "HBIIHDMPAAH")]
    pub hbiihdmpaah: String,

    #[serde(rename = "NEAGDHPBCGF")]
    pub neagdhpbcgf: Vec<i64>,

    #[serde(rename = "HJMNLHNKBLJ")]
    pub hjmnlhnkblj: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Kbjgaekfcgl {
    #[serde(rename = "UI_HOMEWORLD_CHANGE_MOUNTAIN_BUTTON")]
    UiHomeworldChangeMountainButton,

    #[serde(rename = "UI_HOMEWORLD_CUSTOM_BUTTON")]
    UiHomeworldCustomButton,

    #[serde(rename = "UI_HOMEWORLD_CUSTOM_LNLWOODCUT")]
    UiHomeworldCustomLnlwoodcut,

    #[serde(rename = "UI_HOMEWORLD_CUSTOM_MINIASCAPE")]
    UiHomeworldCustomMiniascape,

    #[serde(rename = "UI_HOMEWORLD_CUSTOM_SNOWMANBUTTON")]
    UiHomeworldCustomSnowmanbutton,

    #[serde(rename = "UI_HOMEWORLD_CUSTOM_VINTAGESTALL")]
    UiHomeworldCustomVintagestall,
}
