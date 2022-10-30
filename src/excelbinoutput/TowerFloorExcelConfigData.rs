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

pub type TowerFloorExcelConfigData = Vec<TowerFloorExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TowerFloorExcelConfigDatum {
    #[serde(rename = "floorId")]
    pub floor_id: i64,

    #[serde(rename = "floorIndex")]
    pub floor_index: i64,

    #[serde(rename = "levelGroupId")]
    pub level_group_id: i64,

    #[serde(rename = "overrideMonsterLevel")]
    pub override_monster_level: i64,

    #[serde(rename = "teamNum")]
    pub team_num: i64,

    #[serde(rename = "rewardIdFiveStars")]
    pub reward_id_five_stars: i64,

    #[serde(rename = "rewardIdTenStars")]
    pub reward_id_ten_stars: i64,

    #[serde(rename = "rewardIdFifteenStars")]
    pub reward_id_fifteen_stars: i64,

    #[serde(rename = "rewardIdThreeStars")]
    pub reward_id_three_stars: i64,

    #[serde(rename = "rewardIdSixStars")]
    pub reward_id_six_stars: i64,

    #[serde(rename = "rewardIdNineStars")]
    pub reward_id_nine_stars: i64,

    #[serde(rename = "unlockStarCount")]
    pub unlock_star_count: i64,

    #[serde(rename = "floorLevelConfigId")]
    pub floor_level_config_id: i64,

    #[serde(rename = "bgImage")]
    pub bg_image: String,
}
