-- Your SQL goes here
-- trander.m_exist_country_prefixes definition

CREATE TABLE `m_exist_country_prefixes` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `country_id` bigint unsigned NOT NULL COMMENT 'country id in m_countries',
  `prefix` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'prefix',
  `exist` tinyint(1) NOT NULL DEFAULT '0' COMMENT 'prefix exists',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `m_exist_country_prefixes_country_id_foreign` (`country_id`),
  CONSTRAINT `m_exist_country_prefixes_country_id_foreign` FOREIGN KEY (`country_id`) REFERENCES `m_countries` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6397 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
