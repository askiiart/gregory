#!/usr/bin/env bash
# set env vars for testing
export GREGORY_DB_ADDRESS=postgres
export GREGORY_DB_USER=gregory
export GREGORY_DB_PASSWORD=pass

cargo test
