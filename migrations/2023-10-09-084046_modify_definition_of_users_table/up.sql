-- Your SQL goes here

ALTER TABLE `users`
    DROP COLUMN `check_registration`,
    DROP COLUMN `avatar`,
    MODIFY COLUMN `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    MODIFY COLUMN `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;

