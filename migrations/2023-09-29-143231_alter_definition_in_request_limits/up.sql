-- Your SQL goes here
-- Change the name and comment of the `first_request_at` column
ALTER TABLE `request_limits`
CHANGE COLUMN `first_request_at` `first_requested_at` DATETIME NOT NULL COMMENT 'The first request at';

-- Change the `created_at` column to NOT NULL
ALTER TABLE `request_limits`
MODIFY COLUMN `created_at` TIMESTAMP NOT NULL;
