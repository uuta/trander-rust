#!/bin/bash

# Better support of Docker layer caching in Cargo
# https://hackmd.io/jgkoQ24YRW6i0xWd73S64A
# https://www.reddit.com/r/rust/comments/126xeyx/exploring_the_problem_of_faster_cargo_docker/

source .env

./confirm-db-connection.sh $TEST_DB_SERVICE $DB_PORT $DB_USER $DB_PASS

# It means after passing test, the run command is executed
cargo watch -x "test" -x "run" 
