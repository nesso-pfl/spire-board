use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreatePlayerCard, PlayerCard, UpdatePlayerCard};

/// GET /api/player-cards - List all player cards
pub async fn list_cards(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, PlayerCard>(
        "SELECT id, name, cost, card_type, color, effects, unlock_requirement, base_card_id, created_at
         FROM player_cards
         ORDER BY name"
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(cards) => HttpResponse::Ok().json(cards),
        Err(e) => {
            tracing::error!("Failed to fetch player cards: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch player cards"
            }))
        }
    }
}

/// GET /api/player-cards/{id} - Get a specific player card
pub async fn get_card(pool: web::Data<PgPool>, card_id: web::Path<Uuid>) -> impl Responder {
    match sqlx::query_as::<_, PlayerCard>(
        "SELECT id, name, cost, card_type, color, effects, unlock_requirement, base_card_id, created_at
         FROM player_cards
         WHERE id = $1"
    )
    .bind(card_id.into_inner())
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(card)) => HttpResponse::Ok().json(card),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Player card not found"
        })),
        Err(e) => {
            tracing::error!("Failed to fetch player card: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch player card"
            }))
        }
    }
}

/// POST /api/player-cards - Create a new player card
pub async fn create_card(
    pool: web::Data<PgPool>,
    card: web::Json<CreatePlayerCard>,
) -> impl Responder {
    match sqlx::query_as::<_, PlayerCard>(
        "INSERT INTO player_cards (name, cost, card_type, color, effects, unlock_requirement, base_card_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id, name, cost, card_type, color, effects, unlock_requirement, base_card_id, created_at"
    )
    .bind(&card.name)
    .bind(card.cost)
    .bind(&card.card_type)
    .bind(&card.color)
    .bind(&card.effects)
    .bind(&card.unlock_requirement)
    .bind(card.base_card_id)
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(new_card) => {
            tracing::info!("Created player card: {} (id: {})", new_card.name, new_card.id);
            HttpResponse::Created().json(new_card)
        }
        Err(e) => {
            tracing::error!("Failed to create player card: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create player card"
            }))
        }
    }
}

/// PUT /api/player-cards/{id} - Update a player card
pub async fn update_card(
    pool: web::Data<PgPool>,
    card_id: web::Path<Uuid>,
    updates: web::Json<UpdatePlayerCard>,
) -> impl Responder {
    let card_id = card_id.into_inner();

    // Build dynamic update query based on provided fields
    let mut query = String::from("UPDATE player_cards SET ");
    let mut set_clauses = Vec::new();
    let mut param_count = 1;

    if updates.name.is_some() {
        set_clauses.push(format!("name = ${}", param_count));
        param_count += 1;
    }
    if updates.cost.is_some() {
        set_clauses.push(format!("cost = ${}", param_count));
        param_count += 1;
    }
    if updates.card_type.is_some() {
        set_clauses.push(format!("card_type = ${}", param_count));
        param_count += 1;
    }
    if updates.color.is_some() {
        set_clauses.push(format!("color = ${}", param_count));
        param_count += 1;
    }
    if updates.effects.is_some() {
        set_clauses.push(format!("effects = ${}", param_count));
        param_count += 1;
    }
    if updates.unlock_requirement.is_some() {
        set_clauses.push(format!("unlock_requirement = ${}", param_count));
        param_count += 1;
    }
    if updates.base_card_id.is_some() {
        set_clauses.push(format!("base_card_id = ${}", param_count));
        param_count += 1;
    }

    if set_clauses.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    query.push_str(&set_clauses.join(", "));
    query.push_str(&format!(" WHERE id = ${} RETURNING id, name, cost, card_type, color, effects, unlock_requirement, base_card_id, created_at", param_count));

    let mut query_builder = sqlx::query_as::<_, PlayerCard>(&query);

    if let Some(ref name) = updates.name {
        query_builder = query_builder.bind(name);
    }
    if let Some(cost) = updates.cost {
        query_builder = query_builder.bind(cost);
    }
    if let Some(ref card_type) = updates.card_type {
        query_builder = query_builder.bind(card_type);
    }
    if let Some(ref color) = updates.color {
        query_builder = query_builder.bind(color);
    }
    if let Some(ref effects) = updates.effects {
        query_builder = query_builder.bind(effects);
    }
    if let Some(ref unlock_requirement) = updates.unlock_requirement {
        query_builder = query_builder.bind(unlock_requirement);
    }
    if let Some(base_card_id) = updates.base_card_id {
        query_builder = query_builder.bind(base_card_id);
    }

    query_builder = query_builder.bind(card_id);

    match query_builder.fetch_optional(pool.get_ref()).await {
        Ok(Some(updated_card)) => {
            tracing::info!("Updated player card: {}", card_id);
            HttpResponse::Ok().json(updated_card)
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Player card not found"
        })),
        Err(e) => {
            tracing::error!("Failed to update player card: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update player card"
            }))
        }
    }
}

/// DELETE /api/player-cards/{id} - Delete a player card
pub async fn delete_card(pool: web::Data<PgPool>, card_id: web::Path<Uuid>) -> impl Responder {
    let card_id = card_id.into_inner();

    match sqlx::query("DELETE FROM player_cards WHERE id = $1")
        .bind(card_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                tracing::info!("Deleted player card: {}", card_id);
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Player card not found"
                }))
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete player card: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete player card"
            }))
        }
    }
}

/// GET /api/player-cards/color/{color} - Get cards by color
pub async fn get_cards_by_color(pool: web::Data<PgPool>, color: web::Path<String>) -> impl Responder {
    match sqlx::query_as::<_, PlayerCard>(
        "SELECT id, name, cost, card_type, color, effects, unlock_requirement, base_card_id, created_at
         FROM player_cards
         WHERE color = $1
         ORDER BY name"
    )
    .bind(color.into_inner())
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(cards) => HttpResponse::Ok().json(cards),
        Err(e) => {
            tracing::error!("Failed to fetch player cards by color: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch player cards by color"
            }))
        }
    }
}

/// GET /api/player-cards/upgraded/{base_card_id} - Get upgraded version of a card
pub async fn get_upgraded_card(pool: web::Data<PgPool>, base_card_id: web::Path<Uuid>) -> impl Responder {
    match sqlx::query_as::<_, PlayerCard>(
        "SELECT id, name, cost, card_type, color, effects, unlock_requirement, base_card_id, created_at
         FROM player_cards
         WHERE base_card_id = $1"
    )
    .bind(base_card_id.into_inner())
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(card)) => HttpResponse::Ok().json(card),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Upgraded card not found"
        })),
        Err(e) => {
            tracing::error!("Failed to fetch upgraded card: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch upgraded card"
            }))
        }
    }
}
