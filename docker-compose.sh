#!/bin/bash

source .env

./confirm-db-connection.sh $TEST_DB_SERVICE $DB_PORT $DB_USER $DB_PASS

cargo watch -x check -x test
