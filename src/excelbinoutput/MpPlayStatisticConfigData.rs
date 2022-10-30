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

pub type MpPlayStatisticConfigData = Vec<MpPlayStatisticConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MpPlayStatisticConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MpPlayId")]
    pub mp_play_id: i64,

    #[serde(rename = "settleType")]
    pub settle_type: SettleType,

    #[serde(rename = "settleParam")]
    pub settle_param: Vec<String>,

    #[serde(rename = "clientSyncType")]
    pub client_sync_type: ClientSyncType,

    #[serde(rename = "clientSyncParam")]
    pub client_sync_param: String,

    #[serde(rename = "mpchallengetitleTextMapHash")]
    pub mpchallengetitle_text_map_hash: i64,

    #[serde(rename = "mpchallengeTextMapHash")]
    pub mpchallenge_text_map_hash: i64,

    #[serde(rename = "mpchallengestyleTextMapHash")]
    pub mpchallengestyle_text_map_hash: i64,

    #[serde(rename = "image")]
    pub image: String,

    #[serde(rename = "priority")]
    pub priority: i64,
}

#[derive(Serialize, Deserialize)]
pub enum ClientSyncType {
    #[serde(rename = "MP_PLAY_SETTLE_SYNC_TYPE_MP_GROUP_VARIABLE")]
    MpPlaySettleSyncTypeMpGroupVariable,

    #[serde(rename = "MP_PLAY_SETTLE_SYNC_TYPE_STATISTIC_VALUE")]
    MpPlaySettleSyncTypeStatisticValue,

    #[serde(rename = "MP_PLAY_SETTLE_SYNC_TYPE_WATCHER_PROGRESS")]
    MpPlaySettleSyncTypeWatcherProgress,
}

#[derive(Serialize, Deserialize)]
pub enum SettleType {
    #[serde(rename = "MP_PLAY_SETTLE_TYPE_MONSTER_KILL_NUM")]
    MpPlaySettleTypeMonsterKillNum,

    #[serde(rename = "MP_PLAY_SETTLE_TYPE_MONSTER_TAKE_DEMAGE")]
    MpPlaySettleTypeMonsterTakeDemage,

    #[serde(rename = "MP_PLAY_SETTLE_TYPE_WATCHER_FINISH")]
    MpPlaySettleTypeWatcherFinish,

    #[serde(rename = "MP_PLAY_SETTLE_TYPE_WATCHER_PROGRESS")]
    MpPlaySettleTypeWatcherProgress,
}
