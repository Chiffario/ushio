use rosu_mods::GameMod;
use rosu_v2::model::{
    GameMode, Grade,
    mods::{GameMods, serde::GameModSeed},
    score::{self, Score},
};
use serde::{Deserialize, Serialize, de};
use sqlx::{Database, prelude::FromRow, types::Json};
use time::OffsetDateTime;

/// Trimmed score information for the database
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DatabaseScore {
    pub id: u64,
    pub user_id: u32,
    pub mode: GameMode,
    pub beatmap_id: u32,
    pub replay: bool,
    pub grade: Grade,
    pub accuracy: f32,
    pub max_combo: u32,
    pub build_id: u8,
    pub total_score: u32,
    pub classic_total_score: u64,
    pub legacy_total_score: u32,
    pub total_score_without_mods: Option<u32>,
    pub map_id: u32,
    pub ended_at: OffsetDateTime,
    pub pp: f32,
    pub is_perfect_combo: bool,
    pub legacy_perfect: Option<bool>,
    pub lazer: bool,
    pub data: Data,
}

impl From<Score> for DatabaseScore {
    fn from(value: Score) -> Self {
        Self {
            id: value.id as u64,
            user_id: value.user_id,
            build_id: value.build_id.unwrap_or_default() as u8,
            beatmap_id: value.map_id,
            total_score: value.score,
            classic_total_score: value.classic_score,
            legacy_total_score: value.legacy_score,
            total_score_without_mods: value.total_score_without_mods,
            map_id: value.map_id,
            accuracy: value.accuracy,
            max_combo: value.max_combo,
            ended_at: value.ended_at,
            grade: value.grade,
            pp: value.pp.unwrap_or_default(),
            replay: value.replay,
            legacy_perfect: value.legacy_perfect,
            is_perfect_combo: value.is_perfect_combo,
            mode: value.mode,
            lazer: value.set_on_lazer,
            data: Data {
                max_score_statistics: value.maximum_statistics.into(),
                score_statistics: value.statistics.into(),
                mods: serde_json::value::to_raw_value(&value.mods).unwrap(),
            },
        }
    }
}

/// Extra data (max stats, stats, mods)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Data {
    pub max_score_statistics: ScoreStatistics, // TODO: check which fields should never be filled
    pub score_statistics: ScoreStatistics,
    pub mods: Box<serde_json::value::RawValue>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScoreStatistics {
    #[serde(alias = "count_miss", skip_serializing_if = "Option::is_none")]
    pub miss: Option<u32>,
    #[serde(alias = "count_50", skip_serializing_if = "Option::is_none")]
    pub meh: Option<u32>,
    #[serde(alias = "count_100", skip_serializing_if = "Option::is_none")]
    pub ok: Option<u32>,
    #[serde(alias = "count_katu", skip_serializing_if = "Option::is_none")]
    pub good: Option<u32>,
    #[serde(alias = "count_300", skip_serializing_if = "Option::is_none")]
    pub great: Option<u32>,
    #[serde(alias = "count_geki", skip_serializing_if = "Option::is_none")]
    pub perfect: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_tick_hit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_tick_miss: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_tick_hit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_tick_miss: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_hit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_miss: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_bonus: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_bonus: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slider_tail_hit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combo_break: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_combo_increase: Option<u32>,
}

macro_rules! none_if_default {
    ($value:expr) => {
        if $value == u32::default() {
            None
        } else {
            Some($value)
        }
    };
}
impl From<rosu_v2::model::score::ScoreStatistics> for ScoreStatistics {
    fn from(value: rosu_v2::model::score::ScoreStatistics) -> Self {
        ScoreStatistics {
            miss: none_if_default!(value.miss),
            meh: none_if_default!(value.meh),
            ok: none_if_default!(value.ok),
            good: none_if_default!(value.good),
            great: none_if_default!(value.great),
            perfect: none_if_default!(value.perfect),
            large_tick_hit: none_if_default!(value.large_tick_hit),
            large_tick_miss: none_if_default!(value.large_tick_miss),
            small_tick_hit: none_if_default!(value.small_tick_hit),
            small_tick_miss: none_if_default!(value.small_tick_miss),
            ignore_hit: none_if_default!(value.ignore_hit),
            ignore_miss: none_if_default!(value.ignore_miss),
            large_bonus: none_if_default!(value.large_bonus),
            small_bonus: none_if_default!(value.small_bonus),
            slider_tail_hit: none_if_default!(value.slider_tail_hit),
            combo_break: none_if_default!(value.combo_break),
            legacy_combo_increase: none_if_default!(value.legacy_combo_increase),
        }
    }
}
