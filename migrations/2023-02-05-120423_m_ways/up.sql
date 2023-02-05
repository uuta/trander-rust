-- Your SQL goes here
-- trander.m_ways definition

CREATE TABLE `m_ways` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `way_id` int NOT NULL COMMENT '1: walking, 2: bycicle, 3: car',
  `recommend_frequency` int NOT NULL COMMENT '0: none, 1: middle, 2: high',
  `min_distance` int NOT NULL COMMENT 'min distance',
  `max_distance` int NOT NULL COMMENT 'max distance',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `deleted_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=14 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
