use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{Project, CreateProject, UpdateProject};

pub async fn create_project(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateProject>,
) -> Result<Json<Project>, StatusCode> {
    let project = Project {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        description: payload.description,
        path: payload.path,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };
    
    sqlx::query(
        "INSERT INTO projects (id, name, description, path, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&project.id)
    .bind(&project.name)
    .bind(&project.description)
    .bind(&project.path)
    .bind(&project.created_at)
    .bind(&project.updated_at)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(project))
}

pub async fn list_projects(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Project>>, StatusCode> {
    let projects: Vec<Project> = sqlx::query_as(
        "SELECT * FROM projects ORDER BY updated_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(projects))
}

pub async fn get_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Project>, StatusCode> {
    let project: Project = sqlx::query_as("SELECT * FROM projects WHERE id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(Json(project))
}

pub async fn update_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProject>,
) -> Result<Json<Project>, StatusCode> {
    let mut project: Project = sqlx::query_as("SELECT * FROM projects WHERE id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    if let Some(name) = payload.name {
        project.name = name;
    }
    if let Some(description) = payload.description {
        project.description = Some(description);
    }
    project.updated_at = Utc::now().to_rfc3339();
    
    sqlx::query(
        "UPDATE projects SET name = ?, description = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&project.name)
    .bind(&project.description)
    .bind(&project.updated_at)
    .bind(&id)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(project))
}

pub async fn delete_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM projects WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}
