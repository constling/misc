-- Add down migration script here

ALTER TABLE reports DROP INDEX idx_reports_gameid;
ALTER TABLE reports DROP INDEX idx_reports_state;
ALTER TABLE reports DROP INDEX idx_reports_from;

ALTER TABLE reports  DROP COLUMN state;
ALTER TABLE reports  DROP COLUMN scene;
ALTER TABLE reports  DROP COLUMN game_id;
ALTER TABLE reports  DROP COLUMN updated_at;