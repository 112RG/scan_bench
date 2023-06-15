use crate::tag_helper::AudioMetadata;

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use sea_orm::{PaginatorTrait, QuerySelect};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

// Creates a album entry with belonging to provided artist_id
pub async fn create_album(
    tx: &mut Transaction<'_, Sqlite>,
    cover: Option<String>,
    artist_id: &String,
    metadata: &AudioMetadata,
) -> Result<String, anyhow::Error> {
    let id: String = Uuid::new_v4().to_string();
    let init_time: String = Utc::now().naive_local().to_string();
    sqlx::query(
        "INSERT OR REPLACE INTO albums (
            id,
            path,
            name,
            album_artist,
            album_artist_sort,
            album_artist_credit,
            discogs_albumid,
            discogs_artistid,
            discogs_labelid,
            year,
            composer,
            mb_album_id,
            mb_artist_id,
            album_type,
            label,
            mb_releasegroup_id,
            asin,
            catalog_num,
            script,
            country,
            album_disambig,
            release_group_disambig,
            artist_name,
            cover,
            created_at,
            updated_at,
            artist_id
         )
    VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
    )
    .bind(&id)
    .bind(&metadata.path)
    .bind(&metadata.album_name)
    .bind(&metadata.album_artist)
    .bind(&metadata.album_sort) // ALBUM_ARTIST_SORT
    .bind(&metadata.artist) // ARTIST CREDIT
    /*     .bind(&metadata.genre) // ARTIST CREDIT
    .bind(&metadata.style) // ARTIST CREDIT */
    .bind(&metadata.discogs_albumid)
    .bind(&metadata.discogs_artistid)
    .bind(&metadata.discogs_labelid)
    .bind(metadata.year)
    .bind(&metadata.composer)
    .bind(&metadata.mb_album_id)
    .bind(&metadata.mb_artist_id)
    .bind(&metadata.album_type)
    .bind(&metadata.label)
    .bind(&metadata.mb_releasegroup_id)
    .bind(&metadata.asin)
    .bind(&metadata.catalog_num)
    .bind(&metadata.script)
    /*     .bind(&metadata.language)
     */
    .bind(&metadata.country)
    /*     .bind(&metadata.album_status)
     */
    .bind(&metadata.album_disambig)
    .bind(&metadata.release_group_disambig)
    .bind(&metadata.artist)
    .bind(cover)
    .bind(&init_time)
    .bind(&init_time)
    .bind(artist_id)
    .execute(&mut *tx)
    .await?;
    Ok(id)
}
