-- Add migration script here
PRAGMA foreign_keys=ON;

CREATE TABLE servers (
    server_id INTEGER PRIMARY KEY,
    leaderboard_channel INTEGER,
    leaderboard_message INTEGER,
    landmine_immunity_role INTEGER,
    gambling_enabled INTEGER NOT NULL DEFAULT 0 CHECK(gambling_enabled IN (0,1))
);

CREATE TABLE landmine_channels (
    channel_id INTEGER,
    server_id INTEGER,
    PRIMARY KEY (server_id, channel_id)
    FOREIGN KEY (server_id) REFERENCES servers (server_id)
);

CREATE TABLE balance (
    user_id INTEGER NOT NULL,
    server_id INTEGER NOT NULL,
    baguettes INTEGER NOT NULL,
    PRIMARY KEY (user_id, server_id)
    FOREIGN KEY (server_id) REFERENCES servers (server_id)
);

CREATE TABLE transactions (
    transaction_id INTEGER PRIMARY KEY,
    server_id INTEGER NOT NULL,
    sender_id INTEGER NOT NULL,
    receiver_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (sender_id, server_id) REFERENCES balance (user_id, server_id) ON DELETE CASCADE,
    FOREIGN KEY (receiver_id, server_id) REFERENCES balance (user_id, server_id) ON DELETE CASCADE,
    FOREIGN KEY (server_id) REFERENCES servers (server_id)
);

CREATE TABLE baguette_audit_log (
    action_id INTEGER PRIMARY KEY,
    server_id INTEGER NOT NULL,
    admin_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    action STRING NOT NULL CHECK(action IN ("add_baguettes", "remove_baguettes", "set_baguettes")),
    amount INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id, server_id) REFERENCES balance (user_id, server_id) ON DELETE CASCADE,
    FOREIGN KEY (admin_id, server_id) REFERENCES balance (user_id, server_id) ON DELETE CASCADE,
    FOREIGN KEY (server_id) REFERENCES servers (server_id)
);
