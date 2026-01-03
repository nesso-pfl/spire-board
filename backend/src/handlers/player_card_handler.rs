use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreatePlayerCard, PlayerCard, UpdatePlayerCard};

/// GET /api/player-cards - 全てのプレイヤーカードを一覧取得
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
            tracing::error!("プレイヤーカードの取得に失敗しました: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "プレイヤーカードの取得に失敗しました"
            }))
        }
    }
}

/// GET /api/player-cards/{id} - 特定のプレイヤーカードを取得
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
            "error": "プレイヤーカードが見つかりません"
        })),
        Err(e) => {
            tracing::error!("プレイヤーカードの取得に失敗しました: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "プレイヤーカードの取得に失敗しました"
            }))
        }
    }
}

/// POST /api/player-cards - 新しいプレイヤーカードを作成
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
            tracing::info!("プレイヤーカードを作成しました: {} (id: {})", new_card.name, new_card.id);
            HttpResponse::Created().json(new_card)
        }
        Err(e) => {
            tracing::error!("プレイヤーカードの作成に失敗しました: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "プレイヤーカードの作成に失敗しました"
            }))
        }
    }
}

/// PUT /api/player-cards/{id} - プレイヤーカードを更新
pub async fn update_card(
    pool: web::Data<PgPool>,
    card_id: web::Path<Uuid>,
    updates: web::Json<UpdatePlayerCard>,
) -> impl Responder {
    let card_id = card_id.into_inner();

    // 指定されたフィールドに基づいて動的な更新クエリを構築
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
            "error": "更新するフィールドがありません"
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
            tracing::info!("プレイヤーカードを更新しました: {}", card_id);
            HttpResponse::Ok().json(updated_card)
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "プレイヤーカードが見つかりません"
        })),
        Err(e) => {
            tracing::error!("プレイヤーカードの更新に失敗しました: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "プレイヤーカードの更新に失敗しました"
            }))
        }
    }
}

/// DELETE /api/player-cards/{id} - プレイヤーカードを削除
pub async fn delete_card(pool: web::Data<PgPool>, card_id: web::Path<Uuid>) -> impl Responder {
    let card_id = card_id.into_inner();

    match sqlx::query("DELETE FROM player_cards WHERE id = $1")
        .bind(card_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                tracing::info!("プレイヤーカードを削除しました: {}", card_id);
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "プレイヤーカードが見つかりません"
                }))
            }
        }
        Err(e) => {
            tracing::error!("プレイヤーカードの削除に失敗しました: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "プレイヤーカードの削除に失敗しました"
            }))
        }
    }
}

/// GET /api/player-cards/color/{color} - 色別にカードを取得
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
            tracing::error!("色別のプレイヤーカード取得に失敗しました: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "色別のプレイヤーカード取得に失敗しました"
            }))
        }
    }
}

/// GET /api/player-cards/upgraded/{base_card_id} - カードのアップグレード版を取得
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
            "error": "アップグレード版のカードが見つかりません"
        })),
        Err(e) => {
            tracing::error!("アップグレード版カードの取得に失敗しました: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "アップグレード版カードの取得に失敗しました"
            }))
        }
    }
}
