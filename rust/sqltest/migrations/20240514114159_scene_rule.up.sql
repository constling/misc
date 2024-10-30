-- Add up migration script here
-- Add up migration script here
-- Add up migration script here
-- MySQL
CREATE TABLE IF NOT EXISTS `scene_rule` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `scene` char(128) NOT NULL COMMENT '来源场景',    
    `type` char(36) NOT NULL COMMENT '类型',
    `rule` text COMMENT '规则',
    `desc` text COMMENT '描述信息',
    `created_at` timestamp  DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `scene` (`scene`)) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
