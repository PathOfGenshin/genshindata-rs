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

pub type NewActivityEntryConfigData = Vec<NewActivityEntryConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivityEntryConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityType")]
    pub activity_type: String,

    #[serde(rename = "sortPriority")]
    pub sort_priority: i64,

    #[serde(rename = "tabIcon")]
    pub tab_icon: String,

    #[serde(rename = "NMDKMHMBKBJ")]
    pub nmdkmhmbkbj: Nmdkmhmbkbj,

    #[serde(rename = "DHLIOGLOMKN")]
    pub dhlioglomkn: Dhlioglomkn,

    #[serde(rename = "tabNameTextMapHash")]
    pub tab_name_text_map_hash: i64,

    #[serde(rename = "duration")]
    pub duration: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Dhlioglomkn {
    #[serde(rename = "Eff_UI_GravenInnocence_PushDialog_Unlock")]
    EffUiGravenInnocencePushDialogUnlock,

    #[serde(rename = "Eff_UI_Irodori_PushDialog_Unlock")]
    EffUiIrodoriPushDialogUnlock,

    #[serde(rename = "Eff_UI_LanternRiteV2_PushDialog_Unlock")]
    EffUiLanternRiteV2PushDialogUnlock,

    #[serde(rename = "Eff_UI_MichiaeMatsuri_PushDialog_Unlock")]
    EffUiMichiaeMatsuriPushDialogUnlock,

    #[serde(rename = "Eff_UI_RogueDiary_PushDialog_Unlock")]
    EffUiRogueDiaryPushDialogUnlock,

    #[serde(rename = "Eff_UI_SummerTimeV2_PushDialog_Unlock")]
    EffUiSummerTimeV2PushDialogUnlock,

    #[serde(rename = "Eff_UI_Vintage_PushDialog_Unlock")]
    EffUiVintagePushDialogUnlock,

    #[serde(rename = "")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum Nmdkmhmbkbj {
    #[serde(rename = "ART/UI/Menus/Activity/GravenInnocence/GravenInnocence_PushDialog")]
    ArtUiMenusActivityGravenInnocenceGravenInnocencePushDialog,

    #[serde(rename = "ART/UI/Menus/Activity/Irodori/Irodori_PushDialog")]
    ArtUiMenusActivityIrodoriIrodoriPushDialog,

    #[serde(rename = "ART/UI/Menus/Activity/LanternRiteV2/LanternRiteV2_PushDialog")]
    ArtUiMenusActivityLanternRiteV2LanternRiteV2PushDialog,

    #[serde(rename = "ART/UI/Menus/Activity/MichiaeMatsuri/MichiaeMatsuri_PushDialog")]
    ArtUiMenusActivityMichiaeMatsuriMichiaeMatsuriPushDialog,

    #[serde(rename = "ART/UI/Menus/Activity/RogueDiary/RogueDiary_PushDialog")]
    ArtUiMenusActivityRogueDiaryRogueDiaryPushDialog,

    #[serde(rename = "ART/UI/Menus/Activity/SummerTimeV2/SummerTimeV2_PushDialog")]
    ArtUiMenusActivitySummerTimeV2SummerTimeV2PushDialog,

    #[serde(rename = "ART/UI/Menus/Activity/Vintage/Vintage_PushDialog")]
    ArtUiMenusActivityVintageVintagePushDialog,

    #[serde(rename = "")]
    Empty,
}
