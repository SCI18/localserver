use axum::{
    extract::{Path, State, Multipart},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use std::fs;
use crate::models::FileRecord;

const UPLOAD_DIR: &str = "./uploads";

pub async fn upload_file(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Result<Json<FileRecord>, StatusCode> {
    // Create uploads directory if it doesn't exist
    fs::create_dir_all(UPLOAD_DIR).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let filename = field.file_name()
            .ok_or(StatusCode::BAD_REQUEST)?
            .to_string();
        
        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
        
        let id = Uuid::new_v4().to_string();
        let filepath = format!("{}/{}", UPLOAD_DIR, id);
        
        fs::write(&filepath, &data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let created_at = Utc::now().to_rfc3339();
        
        let file_record = FileRecord {
            id: id.clone(),
            filename: filename.clone(),
            filepath: filepath.clone(),
            size: data.len() as i64,
            mime_type: "application/octet-stream".to_string(),
            project_id: None,
            created_at: created_at.clone(),
        };
        
        sqlx::query(
            "INSERT INTO files (id, filename, filepath, size, mime_type, project_id, created_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&filename)
        .bind(&filepath)
        .bind(file_record.size)
        .bind(&file_record.mime_type)
        .bind(&file_record.project_id)
        .bind(&created_at)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        return Ok(Json(file_record));
    }
    
    Err(StatusCode::BAD_REQUEST)
}

pub async fn get_file(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Vec<u8>, StatusCode> {
    let file: FileRecord = sqlx::query_as("SELECT * FROM files WHERE id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    fs::read(&file.filepath).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_file(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let file: FileRecord = sqlx::query_as("SELECT * FROM files WHERE id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    fs::remove_file(&file.filepath).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    sqlx::query("DELETE FROM files WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_files(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<FileRecord>>, StatusCode> {
    let files: Vec<FileRecord> = sqlx::query_as("SELECT * FROM files ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(files))
}
