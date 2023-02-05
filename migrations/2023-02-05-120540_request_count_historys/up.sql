-- Your SQL goes here
-- trander.request_count_historys definition

CREATE TABLE `request_count_historys` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint unsigned NOT NULL,
  `type_id` int NOT NULL COMMENT '0: Get GeoDB Cities, 1: Get wikidata, 2: Get Yahoo!ローカルサーチAPI, 3: Get 楽天トラベル施設検索API, 4: Get Сurrent weather and forecast',
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `deleted_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `request_count_historys_user_id_index` (`user_id`),
  KEY `request_count_historys_type_id_index` (`type_id`),
  KEY `request_count_historys_created_at_index` (`created_at`),
  KEY `request_count_historys_updated_at_index` (`updated_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
