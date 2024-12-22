#!/bin/sh

set -e

echo "running database migrations"
./migrations

echo "running app"
exec ./chahaein
