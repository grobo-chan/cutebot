-- Add migration script here
CREATE TABLE balance (
    user_id INTEGER PRIMARY KEY,
    money INTEGER NOT NULL
);

CREATE TABLE transactions (
    transaction_id INTEGER PRIMARY KEY,
    sender_id INTEGER NOT NULL,
    receiver_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    FOREIGN KEY (sender_id) REFERENCES balance (user_id),
    FOREIGN KEY (receiver_id) REFERENCES balance (user_id)
);
