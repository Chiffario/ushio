-- Add up migration script here
CREATE TABLE scores (
    id                  BIGINT NOT NULL,
    user_id             BIGINT NOT NULL,
    ruleset_id          SMALLINT NOT NULL,
    beatmap_id          BIGINT NOT NULL,
    has_replay          BOOLEAN NOT NULL DEFAULT FALSE,
    grade               CHAR(2) NOT NULL DEFAULT '',
    accuracy            REAL NOT NULL DEFAULT 0,
    max_combo           INTEGER NOT NULL DEFAULT 0,
    total_score         BIGINT NOT NULL DEFAULT 0,
    classic_total_score BIGINT,
    total_score_without_mods BIGINT,
    is_perfect_combo    BOOLEAN,
    legacy_perfect      BOOLEAN,
    pp                  DOUBLE PRECISION DEFAULT NULL,
    legacy_total_score  BIGINT NOT NULL DEFAULT 0,
    ended_at            TIMESTAMPTZ NOT NULL,
    build_id            SMALLINT DEFAULT NULL,
    lazer               BOOLEAN NOT NULL DEFAULT TRUE,
    data                JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX idx_scores_agg_lookup
ON scores (ruleset_id, build_id, ended_at DESC);

CREATE INDEX idx_scores_user_lookup
ON scores (user_id, ended_at DESC);

CREATE INDEX idx_scores_id_lookup
ON scores (id);

SELECT create_hypertable('scores', by_range('ended_at', INTERVAL '1 day'));
