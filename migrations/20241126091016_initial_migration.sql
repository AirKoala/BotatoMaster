CREATE TABLE content_matcher(
    id INTEGER PRIMARY KEY,
    approved INTEGER NOT NULL DEFAULT FALSE,  -- 0 = false, 1 = true
    author_id INTEGER NOT NULL,
    added_on INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
) STRICT;

CREATE TABLE selector(
    id INTEGER PRIMARY KEY,
    content_matcher_id INTEGER NOT NULL,
    regex TEXT,
    author_id INTEGER,
    channel_id INTEGER,
    guild_id INTEGER,
    FOREIGN KEY(content_matcher_id) REFERENCES content_matcher(id) ON DELETE CASCADE
) STRICT;

CREATE TABLE response(
    id INTEGER PRIMARY KEY,
    response TEXT NOT NULL,
    bias INTEGER NOT NULL DEFAULT 1,
    content_matcher_id INTEGER NOT NULL,
    FOREIGN KEY(content_matcher_id) REFERENCES content_matcher(id) ON DELETE CASCADE
) STRICT;
