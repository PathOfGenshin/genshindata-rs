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

pub type ActivityVintageHuntingExcelConfigData = Vec<ActivityVintageHuntingExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityVintageHuntingExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "CNGBDMFLGGF")]
    pub cngbdmflggf: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "IDDONJAJHNM")]
    pub iddonjajhnm: i64,

    #[serde(rename = "guideQuestId")]
    pub guide_quest_id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "MPEIIPLLCNO")]
    pub mpeiipllcno: i64,

    #[serde(rename = "ACHCCLDFJED")]
    pub achccldfjed: Vec<i64>,

    #[serde(rename = "BAKHECCKLDF")]
    pub bakhecckldf: Vec<i64>,

    #[serde(rename = "AOEBEJKFEBA")]
    pub aoebejkfeba: Vec<i64>,

    #[serde(rename = "tutorialId")]
    pub tutorial_id: i64,

    #[serde(rename = "DFCHALDIOFN")]
    pub dfchaldiofn: i64,

    #[serde(rename = "NGCPNLOBHDP")]
    pub ngcpnlobhdp: i64,

    #[serde(rename = "PBMHBFKDPDP")]
    pub pbmhbfkdpdp: Vec<i64>,

    #[serde(rename = "FKNBALOFCKE")]
    pub fknbalofcke: i64,

    #[serde(rename = "HHKCLJNGIEL")]
    pub hhkcljngiel: i64,
}
