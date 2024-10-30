-- Add up migration script here
-- Add up migration script here
-- MySQL
CREATE TABLE IF NOT EXISTS `reports` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `reporter_id` char(36) NOT NULL COMMENT '举报者',
    `reported_id` char(36) NOT NULL COMMENT '被举报者',
    `reason` ENUM ('verbal_abuse', 'away_from_keyboard', 'cheating', 'disruptive_behavior', 'illegal_name', 'other') COMMENT '举报原因',
    `comment` text COMMENT '举报内容',
    `screenshots` json COMMENT '截图地址，可能包含多张图片',
    `created_at` timestamp  DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `reporter_id` (`reporter_id`),
    KEY `reported_id` (`reported_id`)) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
