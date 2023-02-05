-- trander.google_place_ids definition

CREATE TABLE `google_place_ids` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `place_id` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Place ID',
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Place name',
  `icon` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'Icon path',
  `rating` double(8,2) DEFAULT NULL COMMENT 'Average of rating',
  `photo` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT 'Photo ID',
  `vicinity` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT 'Address',
  `user_ratings_total` int DEFAULT NULL COMMENT 'Sum of ratings',
  `price_level` int DEFAULT NULL COMMENT 'Price Level',
  `lat` decimal(15,7) NOT NULL COMMENT 'Latitude',
  `lng` decimal(15,7) NOT NULL COMMENT 'Longitude',
  `rating_star` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL COMMENT 'Class for rating star',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  `deleted_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `google_place_ids_place_id_index` (`place_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;-- Your SQL goes here
