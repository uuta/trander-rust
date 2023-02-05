-- Your SQL goes here
-- trander.m_countries definition

CREATE TABLE `m_countries` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `country_code` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'country code',
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'country name',
  `exist_in_geo_db_cities` tinyint(1) NOT NULL DEFAULT '0' COMMENT 'exist in GeoDBCities',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `m_countries_country_code_unique` (`country_code`)
) ENGINE=InnoDB AUTO_INCREMENT=247 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
