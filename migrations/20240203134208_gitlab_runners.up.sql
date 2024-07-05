-- Copyright 2024 bmc::labs GmbH. All rights reserved.

CREATE TABLE IF NOT EXISTS gitlab_runners (
    id                INTEGER PRIMARY KEY,
    name              TEXT    NOT NULL,
    url               TEXT    NOT NULL,
    token             TEXT    NOT NULL,
    token_obtained_at TEXT    NOT NULL,
    docker_image      TEXT    NOT NULL,
    UNIQUE(id, url, token)
);
