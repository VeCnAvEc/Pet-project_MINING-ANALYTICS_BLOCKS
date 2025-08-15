CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    txid VARCHAR(64) UNIQUE NOT NULL,
    block_hash VARCHAR(64),
    fee BIGINT NOT NULL DEFAULT 0,
    size INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (block_hash) REFERENCES blocks(hash) ON DELETE SET NULL
);

CREATE INDEX idx_transactions_txid ON transactions(txid);
CREATE INDEX idx_transactions_block_hash ON transactions(block_hash);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);