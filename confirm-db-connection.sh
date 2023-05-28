#!/bin/bash

# confirm-db-connection.sh

set -e

host="$1"
port="$2"
user="$3"
pass="$4"
shift
shift
shift
shift
cmd="$@"

until echo 'SELECT 1' | mysql -h"$host" -P"$port" -u"$user" --password="$pass"; do
  >&2 echo "MySQL is unavailable - sleeping"
  sleep 1
done

>&2 echo "MySQL is up - executing command"
$cmd
./run_migration.sh test

