-- Your SQL goes here
-- trander.m_directions definition

CREATE TABLE `m_directions` (
  `direction_id` int unsigned NOT NULL AUTO_INCREMENT,
  `direction_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '北、北東、東、南東、南、南西、西、北西',
  `min_angle` double(8,2) NOT NULL COMMENT 'min angle',
  `max_angle` double(8,2) NOT NULL COMMENT 'max angle',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `deleted_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`direction_id`)
) ENGINE=InnoDB AUTO_INCREMENT=10 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
