-- Copyright 2024 bmc::labs GmbH. All rights reserved.
CREATE TABLE IF NOT EXISTS gitlab_runners (
    id           TEXT PRIMARY KEY,
    name         TEXT NOT NULL,
    url          TEXT NOT NULL,
    token        TEXT UNIQUE NOT NULL,
    docker_image TEXT NOT NULL,
    tag_list     TEXT NOT NULL,
    run_untagged BOOLEAN NOT NULL
);
