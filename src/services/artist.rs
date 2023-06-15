use chrono::Utc;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

// Creates a artist entry with artist name passed and MusicBrainzArtistId
pub async fn create_artist(
    tx: &mut Transaction<'_, Sqlite>,
    artist_name: &str,
    mb_artist_id: &Option<String>,
) -> Result<String, anyhow::Error> {
    let id: String = Uuid::new_v4().to_string();
    let init_time: String = Utc::now().naive_local().to_string();
    sqlx::query(
        "INSERT OR REPLACE INTO artists (
            id, 
            name,
            mb_artist_id,
            created_at,
            updated_at
         )
    VALUES (?,?,?,?,?)",
    )
    .bind(&id)
    .bind(artist_name)
    .bind(mb_artist_id)
    .bind(&init_time)
    .bind(&init_time)
    .execute(&mut *tx)
    .await?;
    Ok(id)
}
