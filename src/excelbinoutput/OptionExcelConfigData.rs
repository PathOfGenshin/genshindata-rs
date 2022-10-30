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

pub type OptionExcelConfigData = Vec<OptionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OptionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "contentTextMapHash")]
    pub content_text_map_hash: i64,

    #[serde(rename = "inteeIconName")]
    pub intee_icon_name: InteeIconName,

    #[serde(rename = "gusetInteeButtonShow")]
    pub guset_intee_button_show: Option<bool>,

    #[serde(rename = "CBPOAHNPDGM")]
    pub cbpoahnpdgm: Option<f64>,

    #[serde(rename = "cancelOnlineMatch")]
    pub cancel_online_match: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum InteeIconName {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Icon_Intee_BuildScore")]
    UiIconInteeBuildScore,

    #[serde(rename = "UI_Icon_Intee_Investigation")]
    UiIconInteeInvestigation,

    #[serde(rename = "UI_Icon_Intee_ItemIcon_109")]
    UiIconInteeItemIcon109,

    #[serde(rename = "UI_Icon_Intee_Mechanism")]
    UiIconInteeMechanism,

    #[serde(rename = "UI_Icon_Intee_Mechanism_MP")]
    UiIconInteeMechanismMp,

    #[serde(rename = "UI_Icon_Intee_PickUp")]
    UiIconInteePickUp,

    #[serde(rename = "UI_Icon_Intee_Talk")]
    UiIconInteeTalk,

    #[serde(rename = "UI_Icon_Intee_TsurumiSigil_01_A")]
    UiIconInteeTsurumiSigil01_A,

    #[serde(rename = "UI_Icon_Intee_TsurumiSigil_01_B")]
    UiIconInteeTsurumiSigil01_B,

    #[serde(rename = "UI_Icon_Intee_TsurumiSigil_01_C")]
    UiIconInteeTsurumiSigil01_C,

    #[serde(rename = "UI_Icon_Intee_TsurumiSigil_01_D")]
    UiIconInteeTsurumiSigil01_D,

    #[serde(rename = "UI_Icon_Intee_TsurumiSigil_01_E")]
    UiIconInteeTsurumiSigil01_E,

    #[serde(rename = "UI_Icon_Intee_TsurumiSigil_01_F")]
    UiIconInteeTsurumiSigil01_F,

    #[serde(rename = "UI_Icon_Intee_TsurumiSigil_01_G")]
    UiIconInteeTsurumiSigil01_G,

    #[serde(rename = "UI_Icon_Quest_Once")]
    UiIconQuestOnce,
}
