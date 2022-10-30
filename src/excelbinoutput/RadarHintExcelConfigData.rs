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

pub type RadarHintExcelConfigData = Vec<RadarHintExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RadarHintExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "IOHBJKHBDNM")]
    pub iohbjkhbdnm: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "radius")]
    pub radius: f64,

    #[serde(rename = "effectName")]
    pub effect_name: EffectName,

    #[serde(rename = "iconName")]
    pub icon_name: String,

    #[serde(rename = "audioName")]
    pub audio_name: AudioName,

    #[serde(rename = "offsetRadius")]
    pub offset_radius: Option<f64>,

    #[serde(rename = "areaRadius")]
    pub area_radius: Option<f64>,

    #[serde(rename = "JPHAFHECAKN")]
    pub jphafhecakn: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum AudioName {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Play_ui_feedback_windCrystal_appear")]
    PlayUiFeedbackWindCrystalAppear,
}

#[derive(Serialize, Deserialize)]
pub enum EffectName {
    #[serde(rename = "Eff_UI_Mark_WindCrystal")]
    EffUiMarkWindCrystal,

    #[serde(rename = "")]
    Empty,
}
