#!/bin/bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "ERROR: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then 
    echo >&2 "ERROR: sqlx is not installed."
    exit 1
fi

DB_NETWORK="${DOCKER_NETWORK:=default}"
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=todo}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

if [[ -z "${SKIP_DOCKER}" ]]; then

    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        --name pg \
        --network "${DB_NETWORK}" \
        -d postgres \
        postgres -N 1000

fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do 
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_URL

echo "DATABASE_URL=$DATABASE_URL" > .env

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go"
