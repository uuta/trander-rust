-- Your SQL goes here
-- trander.request_limits definition

CREATE TABLE `request_limits` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint unsigned NOT NULL,
  `request_limit` bigint unsigned NOT NULL DEFAULT '10' COMMENT 'Number of request limit',
  `first_request_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'The first request at',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `request_limits_user_id_index` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
