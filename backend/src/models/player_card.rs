use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

/// Player card definition for deck-building gameplay
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlayerCard {
    pub id: Uuid,
    pub name: String,
    pub cost: i32,
    pub card_type: String,
    pub color: String,
    /// Array of effects in JSON format
    /// Each effect has: type, target, value, and optional metadata
    /// Example: [{"type": "block", "value": 3, "target": "any_player"}]
    pub effects: JsonValue,
    pub unlock_requirement: Option<String>,
    /// NULL for base cards, references base card ID for upgraded versions
    pub base_card_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Data structure for creating a new player card
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

/// Data structure for updating a player card
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
