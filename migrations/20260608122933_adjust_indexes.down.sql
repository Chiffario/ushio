-- restore initial index shapes
DROP INDEX idx_scores_agg_lookup;

CREATE INDEX idx_scores_agg_lookup
ON scores (ruleset_id, build_id, ended_at DESC);
