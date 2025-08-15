CREATE TABLE blocks (
    id SERIAL PRIMARY KEY,
    hash VARCHAR(64) UNIQUE NOT NULL,
    height BIGINT UNIQUE NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    transactions_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_blocks_hash ON blocks(hash);
CREATE INDEX idx_blocks_height ON blocks(height);
CREATE INDEX idx_blocks_timestamp ON blocks(timestamp);