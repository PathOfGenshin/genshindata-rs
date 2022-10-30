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

pub type WidgetGeneralExcelConfigData = Vec<WidgetGeneralExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WidgetGeneralExcelConfigDatum {
    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "canUseInOtherWorld")]
    pub can_use_in_other_world: Option<bool>,

    #[serde(rename = "BLEDFGMHJHM")]
    pub bledfgmhjhm: Vec<i64>,

    #[serde(rename = "forbiddenDungeonList")]
    pub forbidden_dungeon_list: Vec<i64>,

    #[serde(rename = "CKDHFEEMJKG")]
    pub ckdhfeemjkg: Vec<i64>,

    #[serde(rename = "canUseInDungeon")]
    pub can_use_in_dungeon: Option<bool>,

    #[serde(rename = "canUseInHomeworld")]
    pub can_use_in_homeworld: Option<bool>,

    #[serde(rename = "canUseInRoom")]
    pub can_use_in_room: Option<bool>,

    #[serde(rename = "canUseInLimitRegion")]
    pub can_use_in_limit_region: Option<bool>,

    #[serde(rename = "canUseWhenFight")]
    pub can_use_when_fight: Option<bool>,

    #[serde(rename = "canUseInUnNormalMoveState")]
    pub can_use_in_un_normal_move_state: Option<bool>,

    #[serde(rename = "canUseInAvatarFocus")]
    pub can_use_in_avatar_focus: Option<bool>,

    #[serde(rename = "canUseWhenCurrentAvatarDead")]
    pub can_use_when_current_avatar_dead: Option<bool>,

    #[serde(rename = "BJJAHOACPGA")]
    pub bjjahoacpga: Option<String>,

    #[serde(rename = "NLCALGGDIII")]
    pub nlcalggdiii: Option<bool>,
}
