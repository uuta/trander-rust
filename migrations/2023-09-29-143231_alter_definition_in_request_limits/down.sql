-- This file should undo anything in `up.sql`
-- Revert the name and comment of the `first_requested_at` column back to `first_request_at`
ALTER TABLE `request_limits`
CHANGE COLUMN `first_requested_at` `first_request_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'The first request at';

-- Revert the `created_at` column back to allowing NULL values
ALTER TABLE `request_limits`
MODIFY COLUMN `created_at` TIMESTAMP NULL DEFAULT NULL;

