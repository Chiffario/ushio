-- use a more proper field than build_id
DROP INDEX idx_scores_agg_lookup;

CREATE INDEX idx_scores_agg_lookup
ON scores (ruleset_id, lazer, ended_at DESC);
