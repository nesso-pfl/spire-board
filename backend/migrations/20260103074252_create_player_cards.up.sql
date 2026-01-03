-- player_cardsテーブルを作成
-- このテーブルは攻撃、スキル、パワーを含むプレイヤーカード定義を保存

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

-- 一般的なクエリ用のインデックス
CREATE INDEX idx_player_cards_base_card_id ON player_cards(base_card_id);
CREATE INDEX idx_player_cards_color ON player_cards(color);
CREATE INDEX idx_player_cards_type ON player_cards(card_type);
CREATE INDEX idx_player_cards_cost ON player_cards(cost);

-- 効果検索用のインデックス
CREATE INDEX idx_player_cards_effects ON player_cards USING GIN (effects);

-- ドキュメント用のコメント
COMMENT ON TABLE player_cards IS 'デッキ構築ゲームプレイ用のプレイヤーカード定義';
COMMENT ON COLUMN player_cards.effects IS 'JSON形式の効果配列。各効果には type, target, value と任意の metadata が含まれる';
COMMENT ON COLUMN player_cards.base_card_id IS 'ベースカードの場合はNULL、アップグレード版の場合はベースカードを参照';
