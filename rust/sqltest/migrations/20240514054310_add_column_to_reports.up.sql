-- Add up migration script here

ALTER TABLE `reports`
ADD COLUMN state  ENUM ('Init', 'Handling', 'Complete', 'Invalid') NOT NULL DEFAULT 'Init' COMMENT '处理状态' after screenshots;

ALTER TABLE `reports`
ADD COLUMN scene  char(36) COMMENT '上报来源' after reported_id;

ALTER TABLE `reports`
ADD COLUMN game_id char(36) COMMENT '游戏id' after reported_id;

ALTER TABLE `reports`
ADD COLUMN updated_at timestamp not NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间' after created_at;


CREATE INDEX idx_reports_gameid ON reports (game_id);
CREATE INDEX idx_reports_state ON reports (state);
CREATE INDEX idx_reports_scene ON reports (scene);
