-- Create player_cards table
-- This table stores player card definitions including attacks, skills, and powers

CREATE TABLE player_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    cost INTEGER NOT NULL CHECK (cost >= 0),
    card_type VARCHAR(20) NOT NULL CHECK (card_type IN ('attack', 'skill', 'power')),
    color VARCHAR(20) NOT NULL CHECK (color IN ('red', 'gray', 'black', 'gold')),
    effects JSONB NOT NULL DEFAULT '[]'::jsonb,
    unlock_requirement VARCHAR(10) CHECK (unlock_requirement IN ('1', '2', '3', 'C')),
    base_card_id UUID REFERENCES player_cards(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for common queries
CREATE INDEX idx_player_cards_base_card_id ON player_cards(base_card_id);
CREATE INDEX idx_player_cards_color ON player_cards(color);
CREATE INDEX idx_player_cards_type ON player_cards(card_type);
CREATE INDEX idx_player_cards_cost ON player_cards(cost);

-- Index for searching effects
CREATE INDEX idx_player_cards_effects ON player_cards USING GIN (effects);

-- Comments for documentation
COMMENT ON TABLE player_cards IS 'Player card definitions for deck-building gameplay';
COMMENT ON COLUMN player_cards.effects IS 'Array of effects in JSON format. Each effect has type, target, value, and optional metadata';
COMMENT ON COLUMN player_cards.base_card_id IS 'NULL for base cards, references base card for upgraded versions';
