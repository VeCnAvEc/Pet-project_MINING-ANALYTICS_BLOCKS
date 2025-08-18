ALTER TABLE transactions
ADD COLUMN is_coinbase BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN main_reward BIGINT,
ADD COLUMN miner_address VARCHAR(255),
ADD COLUMN full_reward BIGINT,
ADD COLUMN guessed_miner VARCHAR(255);

-- Индексы
CREATE INDEX idx_transactions_is_coinbase ON transactions(is_coinbase);
CREATE INDEX idx_transactions_block_coinbase ON transactions(block_hash, is_coinbase);