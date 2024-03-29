-- Copyright 2024 bmc::labs GmbH. All rights reserved.
CREATE TABLE IF NOT EXISTS runners (
    id           INTEGER PRIMARY KEY NOT NULL,
    url          TEXT                NOT NULL,
    token        TEXT                NOT NULL,
    description  TEXT                NOT NULL,
    image        TEXT                NOT NULL,
    tag_list     TEXT                NOT NULL,
    run_untagged BOOLEAN             NOT NULL
);
