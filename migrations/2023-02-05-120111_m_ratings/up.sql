-- Your SQL goes here
-- trander.m_ratings definition

CREATE TABLE `m_ratings` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `class_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Class name to display stars',
  `min` double(8,2) NOT NULL COMMENT 'Rating of minimum',
  `max` double(8,2) NOT NULL COMMENT 'Rating of maximum',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `deleted_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
