#!/bin/bash

source .env

./confirm-db-connection.sh $TEST_DB_SERVICE $DB_PORT $DB_USER $DB_PASS

# It means after passing test, the run command is executed
cargo watch -x "test" -x "run" 
