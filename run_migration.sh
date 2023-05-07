#!/bin/bash

# Load the .env file
source .env

# Check the input argument and set the DB_URL variable accordingly
if [ "$1" == "test" ]; then
  export DATABASE_URL=$TEST_DATABASE_URL
elif [ "$1" == "dev" ]; then
  export DATABASE_URL=$DEV_DATABASE_URL
else
  echo "Invalid environment argument. Please use 'test' or 'dev'."
  exit 1
fi

# Run the migration
diesel migration run
