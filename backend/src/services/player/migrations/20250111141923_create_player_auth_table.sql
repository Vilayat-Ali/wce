-- PLAYER AUTH
CREATE TABLE IF NOT EXISTS player_auth (
    player_id INTEGER PRIMARY KEY,
    refresh_token TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_player_id ON player_auth (refresh_token);

CREATE TRIGGER update_player_updated_at
BEFORE UPDATE ON player_auth
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();