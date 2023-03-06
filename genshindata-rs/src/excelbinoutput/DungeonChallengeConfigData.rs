// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DungeonChallengeConfigData = Vec<DungeonChallengeConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonChallengeConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "targetTextTemplateTextMapHash")]
    pub target_text_template_text_map_hash: i64,

    #[serde(rename = "subTargetTextTemplateTextMapHash")]
    pub sub_target_text_template_text_map_hash: i64,

    #[serde(rename = "progressTextTemplateTextMapHash")]
    pub progress_text_template_text_map_hash: i64,

    #[serde(rename = "subProgressTextTemplateTextMapHash")]
    pub sub_progress_text_template_text_map_hash: i64,

    #[serde(rename = "iconPath")]
    pub icon_path: String,

    #[serde(rename = "challengeType")]
    pub challenge_type: ChallengeType,

    #[serde(rename = "HHIPHKLJKEL")]
    pub hhiphkljkel: Vec<String>,

    #[serde(rename = "noSuccessHint")]
    pub no_success_hint: Option<bool>,

    #[serde(rename = "interruptButtonType")]
    pub interrupt_button_type: Option<InterruptButtonType>,

    #[serde(rename = "noFailHint")]
    pub no_fail_hint: Option<bool>,

    #[serde(rename = "isBlockTopTimer")]
    pub is_block_top_timer: Option<bool>,

    #[serde(rename = "subChallengeFadeOutRule")]
    pub sub_challenge_fade_out_rule: Option<SubChallengeFadeOutRule>,

    #[serde(rename = "subChallengeFadeOutDelayTime")]
    pub sub_challenge_fade_out_delay_time: Option<f64>,

    #[serde(rename = "subChallengeBannerRule")]
    pub sub_challenge_banner_rule: Option<String>,

    #[serde(rename = "LFGFBDFELOD")]
    pub lfgfbdfelod: Option<bool>,

    #[serde(rename = "recordType")]
    pub record_type: Option<String>,

    #[serde(rename = "NINJPJKPFBJ")]
    pub ninjpjkpfbj: Option<bool>,

    #[serde(rename = "isSuccessWhenNotSettled")]
    pub is_success_when_not_settled: Option<bool>,

    #[serde(rename = "activitySkillID")]
    pub activity_skill_id: Option<i64>,

    #[serde(rename = "isForwardTiming")]
    pub is_forward_timing: Option<bool>,

    #[serde(rename = "KDJPDOBBLCN")]
    pub kdjpdobblcn: Option<bool>,

    #[serde(rename = "BDDBEPKMOLJ")]
    pub bddbepkmolj: Option<String>,

    #[serde(rename = "AINDJOHBANE")]
    pub aindjohbane: Option<String>,

    #[serde(rename = "MLPJKFMMLIA")]
    pub mlpjkfmmlia: Option<String>,

    #[serde(rename = "ICGCEPMJLJG")]
    pub icgcepmjljg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeType {
    #[serde(rename = "CHALLENGE_CRYSTAL_ELEMENT_REACTION_COUNT")]
    ChallengeCrystalElementReactionCount,

    #[serde(rename = "CHALLENGE_DIE_LESS_IN_TIME")]
    ChallengeDieLessInTime,

    #[serde(rename = "CHALLENGE_ELEMENT_REACTION_COUNT")]
    ChallengeElementReactionCount,

    #[serde(rename = "CHALLENGE_FATHER_SUCC_IN_TIME")]
    ChallengeFatherSuccInTime,

    #[serde(rename = "CHALLENGE_FREEZE_ENEMY_IN_TIME")]
    ChallengeFreezeEnemyInTime,

    #[serde(rename = "CHALLENGE_GUARD_HP")]
    ChallengeGuardHp,

    #[serde(rename = "CHALLENGE_KILL_COUNT")]
    ChallengeKillCount,

    #[serde(rename = "CHALLENGE_KILL_COUNT_FAST")]
    ChallengeKillCountFast,

    #[serde(rename = "CHALLENGE_KILL_COUNT_FROZEN_LESS")]
    ChallengeKillCountFrozenLess,

    #[serde(rename = "CHALLENGE_KILL_COUNT_GUARD_HP")]
    ChallengeKillCountGuardHp,

    #[serde(rename = "CHALLENGE_KILL_COUNT_IN_TIME")]
    ChallengeKillCountInTime,

    #[serde(rename = "CHALLENGE_KILL_MONSTER_IN_TIME")]
    ChallengeKillMonsterInTime,

    #[serde(rename = "CHALLENGE_LUA_IN_TIME")]
    ChallengeLuaInTime,

    #[serde(rename = "CHALLENGE_MONSTER_DAMAGE_COUNT")]
    ChallengeMonsterDamageCount,

    #[serde(rename = "CHALLENGE_SHEILD_ABSORB_DAMAGE_COUNT")]
    ChallengeSheildAbsorbDamageCount,

    #[serde(rename = "CHALLENGE_SURVIVE")]
    ChallengeSurvive,

    #[serde(rename = "CHALLENGE_SURVIVE_IN_TIME")]
    ChallengeSurviveInTime,

    #[serde(rename = "CHALLENGE_SWIRL_ELEMENT_REACTION_COUNT")]
    ChallengeSwirlElementReactionCount,

    #[serde(rename = "CHALLENGE_TIME_FLY")]
    ChallengeTimeFly,

    #[serde(rename = "CHALLENGE_TRIGGER2_AVOID_TRIGGER1")]
    ChallengeTrigger2AvoidTrigger1,

    #[serde(rename = "CHALLENGE_TRIGGER_COUNT")]
    ChallengeTriggerCount,

    #[serde(rename = "CHALLENGE_TRIGGER_IN_TIME")]
    ChallengeTriggerInTime,

    #[serde(rename = "CHALLENGE_TRIGGER_IN_TIME_FLY")]
    ChallengeTriggerInTimeFly,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InterruptButtonType {
    #[serde(rename = "INTERRUPT_BUTTON_TYPE_ALL")]
    InterruptButtonTypeAll,

    #[serde(rename = "INTERRUPT_BUTTON_TYPE_HOST")]
    InterruptButtonTypeHost,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubChallengeFadeOutRule {
    #[serde(rename = "SUBCHALLENGE_FADEOUT_TYPE_FAIL")]
    SubchallengeFadeoutTypeFail,

    #[serde(rename = "SUBCHALLENGE_FADEOUT_TYPE_FINISH")]
    SubchallengeFadeoutTypeFinish,

    #[serde(rename = "SUBCHALLENGE_FADEOUT_TYPE_SUCCESS")]
    SubchallengeFadeoutTypeSuccess,
}

pub fn load() -> Result<DungeonChallengeConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "DungeonChallengeConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
