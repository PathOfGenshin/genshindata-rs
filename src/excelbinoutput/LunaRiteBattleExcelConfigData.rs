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

pub type LunaRiteBattleExcelConfigData = Vec<LunaRiteBattleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LunaRiteBattleExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "JDPGGCCINLP")]
    pub jdpggccinlp: Jdpggccinlp,

    #[serde(rename = "IBHNNLLDCKG")]
    pub ibhnnlldckg: Option<i64>,

    #[serde(rename = "NKMBOEBPONE")]
    pub nkmboebpone: i64,

    #[serde(rename = "rewardID")]
    pub reward_id: i64,

    #[serde(rename = "MMIGMKDOAOL")]
    pub mmigmkdoaol: String,

    #[serde(rename = "PLHNOFMNEHM")]
    pub plhnofmnehm: String,

    #[serde(rename = "BPONEFHCAFP")]
    pub bponefhcafp: String,

    #[serde(rename = "KMHINBKOEKF")]
    pub kmhinbkoekf: i64,

    #[serde(rename = "OLPHBBHIMPM")]
    pub olphbbhimpm: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Jdpggccinlp {
    #[serde(rename = "LUNA_RITE_REGION_TYPE_DRAGONSPINE")]
    LunaRiteRegionTypeDragonspine,

    #[serde(rename = "LUNA_RITE_REGION_TYPE_LIYUE")]
    LunaRiteRegionTypeLiyue,

    #[serde(rename = "LUNA_RITE_REGION_TYPE_MENGDE")]
    LunaRiteRegionTypeMengde,
}
