#!/usr/bin/env bash

rm -rf ./data/
mkdir -p ./data/{fedora-repo,librewolf,other-workspace}

mkdir -p ./dev/{pgadmin,gregory-pg}
chmod -R 777 ./dev/pgadmin

podman-compose down
podman-compose -f podman-compose.dev.yml up -d

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