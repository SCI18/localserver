use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{Snippet, CreateSnippet};

pub async fn create_snippet(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateSnippet>,
) -> Result<Json<Snippet>, StatusCode> {
    let tags = payload.tags.map(|t| t.join(","));
    
    let snippet = Snippet {
        id: Uuid::new_v4().to_string(),
        title: payload.title,
        language: payload.language,
        code: payload.code,
        description: payload.description,
        tags,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };
    
    sqlx::query(
        "INSERT INTO snippets (id, title, language, code, description, tags, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&snippet.id)
    .bind(&snippet.title)
    .bind(&snippet.language)
    .bind(&snippet.code)
    .bind(&snippet.description)
    .bind(&snippet.tags)
    .bind(&snippet.created_at)
    .bind(&snippet.updated_at)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(snippet))
}

pub async fn list_snippets(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Snippet>>, StatusCode> {
    let snippets: Vec<Snippet> = sqlx::query_as(
        "SELECT * FROM snippets ORDER BY updated_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(snippets))
}

pub async fn get_snippet(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Snippet>, StatusCode> {
    let snippet: Snippet = sqlx::query_as("SELECT * FROM snippets WHERE id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(Json(snippet))
}

pub async fn delete_snippet(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM snippets WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}
