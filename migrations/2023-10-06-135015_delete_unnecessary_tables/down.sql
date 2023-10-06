-- This file should undo anything in `up.sql`
-- trander.m_countries definition

CREATE TABLE `m_countries` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `country_code` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'country code',
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'country name',
  `exist_in_geo_db_cities` tinyint(1) NOT NULL DEFAULT '0' COMMENT 'exist in GeoDBCities',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `m_countries_country_code_unique` (`country_code`)
) ENGINE=InnoDB AUTO_INCREMENT=247 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- trander.m_directions definition

CREATE TABLE `m_directions` (
  `direction_id` int unsigned NOT NULL AUTO_INCREMENT,
  `direction_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '北、北東、東、南東、南、南西、西、北西',
  `min_angle` double(8,2) NOT NULL COMMENT 'min angle',
  `max_angle` double(8,2) NOT NULL COMMENT 'max angle',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `deleted_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`direction_id`)
) ENGINE=InnoDB AUTO_INCREMENT=10 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- trander.m_exist_country_prefixes definition

CREATE TABLE `m_exist_country_prefixes` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `country_id` bigint unsigned NOT NULL COMMENT 'country id in m_countries',
  `prefix` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'prefix',
  `exist` tinyint(1) NOT NULL DEFAULT '0' COMMENT 'prefix exists',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `m_exist_country_prefixes_country_id_foreign` (`country_id`),
  CONSTRAINT `m_exist_country_prefixes_country_id_foreign` FOREIGN KEY (`country_id`) REFERENCES `m_countries` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6397 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

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
