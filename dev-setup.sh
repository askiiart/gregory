#!/usr/bin/env bash
set -ex

command_exists() { type "$1" &>/dev/null; }

./test.sh

rm -rf ./data/
mkdir -p ./data/{fedora-repo,librewolf,other-workspace}

mkdir -p ./dev/{pgadmin,gregory-pg}
chmod -R 777 ./dev/pgadmin

if command_exists "docker-compose"; then
    docker-compose -f podman-compose.dev.yml down
    docker-compose -f podman-compose.dev.yml up -d
elif command_exists "podman-compose"; then
    podman-compose -f podman-compose.dev.yml down
    podman-compose -f podman-compose.dev.yml up -d
else
    echo "[ERROR] neither docker-compose nor podman-compose were found"
    exit 127
fi

echo "
---
"
echo 'pgadmin login:
    Email: "a@a.aaa"
    Password: "pass"
    '
echo 'pgadmin settings:
    Hostname: "postgres"
    Username: "gregory"
    Password: "pass"'
