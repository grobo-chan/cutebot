-- Add migration script here
CREATE TABLE servers (
    server_id INTEGER PRIMARY KEY,
    leaderboard_channel INTEGER,
    gambling_enabled INTEGER NOT NULL DEFAULT 0 CHECK(gambling_enabled IN (0,1))
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
    FOREIGN KEY (sender_id, server_id) REFERENCES balance (user_id, server_id),
    FOREIGN KEY (receiver_id, server_id) REFERENCES balance (user_id, server_id),
    FOREIGN KEY (server_id) REFERENCES servers (server_id)
);
