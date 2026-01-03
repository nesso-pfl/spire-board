use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

/// デッキ構築ゲームプレイ用のプレイヤーカード定義
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlayerCard {
    pub id: Uuid,
    pub name: String,
    pub cost: i32,
    pub card_type: String,
    pub color: String,
    /// JSON形式の効果配列
    /// 各効果には type, target, value と任意の metadata が含まれる
    /// 例: [{"type": "block", "value": 3, "target": "any_player"}]
    pub effects: JsonValue,
    pub unlock_requirement: Option<String>,
    /// ベースカードの場合はNULL、アップグレード版の場合はベースカードIDを参照
    pub base_card_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// 新しいプレイヤーカードを作成するためのデータ構造
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlayerCard {
    pub name: String,
    pub cost: i32,
    pub card_type: String,
    pub color: String,
    pub effects: JsonValue,
    pub unlock_requirement: Option<String>,
    pub base_card_id: Option<Uuid>,
}

/// プレイヤーカードを更新するためのデータ構造
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePlayerCard {
    pub name: Option<String>,
    pub cost: Option<i32>,
    pub card_type: Option<String>,
    pub color: Option<String>,
    pub effects: Option<JsonValue>,
    pub unlock_requirement: Option<String>,
    pub base_card_id: Option<Uuid>,
}
