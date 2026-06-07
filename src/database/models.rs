use rosu_v2::model::{
    GameMode, Grade,
    mods::GameMods,
    score::{self, Score, ScoreStatistics},
};
use serde::Serialize;
use sqlx::{Database, prelude::FromRow, types::Json};
use time::OffsetDateTime;

/// Trimmed score information for the database
#[derive(Debug, FromRow)]
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
                max_score_statistics: value.maximum_statistics,
                score_statistics: value.statistics,
                mods: value.mods,
            },
        }
    }
}

/// Extra data (max stats, stats, mods)
#[derive(Serialize, Debug, Clone)]
pub struct Data {
    pub max_score_statistics: ScoreStatistics, // TODO: check which fields should never be filled
    pub score_statistics: ScoreStatistics,
    pub mods: GameMods,
}
