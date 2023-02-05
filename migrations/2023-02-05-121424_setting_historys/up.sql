-- Your SQL goes here
-- trander.setting_historys definition

CREATE TABLE `setting_historys` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `setting_id` bigint unsigned NOT NULL,
  `min_distance` int NOT NULL,
  `max_distance` int NOT NULL,
  `direction_type` smallint NOT NULL COMMENT '0. None, 1. North, 2. East, 3. South, 4. West',
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `deleted_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `setting_historys_setting_id_index` (`setting_id`),
  CONSTRAINT `setting_historys_setting_id_foreign` FOREIGN KEY (`setting_id`) REFERENCES `settings` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
