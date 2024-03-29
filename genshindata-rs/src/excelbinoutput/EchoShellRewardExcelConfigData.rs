/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EchoShellRewardExcelConfigData = Vec<EchoShellRewardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EchoShellRewardExcelConfigDatum {
    pub id: i64,
    pub reward_id: i64,
    pub shell_count: i64,
    pub show_at_top: Option<bool>,
}
