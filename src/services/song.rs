use chrono::Utc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sqlx::{sqlite::SqliteQueryResult, Sqlite, Transaction};

use uuid::Uuid;

use crate::tag_helper::AudioMetadata;

// // Creates a song entry with with the passed album_id and AudioMetadata block
pub async fn create_song(
    tx: &mut Transaction<'_, Sqlite>,
    album_id: &str,
    metadata: &AudioMetadata,
) -> Result<SqliteQueryResult, anyhow::Error> {
    let id: Uuid = Uuid::new_v4();
    let init_time: String = Utc::now().naive_local().to_string();
    Ok(sqlx::query(
        "INSERT OR REPLACE INTO songs (
            id,
            path,
            title,
            artist,
            artist_sort,
            album_name,
            album_artist,
            album_sort,
            discogs_albumid,
            discogs_artistid,
            discogs_labelid,
            lyricist,
            composer,
            composer_sort,
            work,
            mb_workid,
            arranger,
            grouping,
            year,
            lyrics,
            comments,
            bpm,
            comp,
            mb_track_id,
            mb_album_id,
            mb_artist_id,
            mb_albumartist_id,
            mb_releasetrack_id,
            mb_releasegroup_id,
            track_disambig,
            album_type,
            acoustid_fingerprint,
            acoustid_id,
            asin,
            isrc,
            catalog_num,
            script,
            country,
            album_status,
            media,
            album_disambig,
            release_group_disambig,
            encoder,
            original_year,
            initial_key,
            bit_rate,
            encoder_settings,
            track,
            disk,
            length,
            label,
            sample_rate,
            bits_per_sample,
            created_at,
            updated_at,
            album_id,
            liked
         )
    VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
    )
    .bind(id.to_string())
    .bind(&metadata.path)
    .bind(&metadata.name)
    .bind(&metadata.artist)
    .bind(&metadata.artist_sort)
    .bind(&metadata.album_name)
    .bind(&metadata.album_artist)
    .bind(&metadata.album_sort)
    /*     .bind(
            &metadata
                .genre
                .into_iter()
                .map(|i| String::from("{:?}", i))
                .collect::<String>(),
        ) */
    /*     .bind(&metadata.style)
     */
    .bind(&metadata.discogs_albumid)
    .bind(&metadata.discogs_artistid)
    .bind(&metadata.discogs_labelid)
    .bind(&metadata.lyricist)
    .bind(&metadata.composer)
    .bind(&metadata.composer_sort)
    .bind(&metadata.work)
    .bind(&metadata.mb_workid)
    /*     .bind(&metadata.work_disambig)
     */
    .bind(&metadata.arranger)
    .bind(&metadata.grouping)
    .bind(metadata.year)
    .bind(&metadata.lyrics)
    .bind(&metadata.comments)
    .bind(&metadata.bpm)
    .bind(&metadata.compilation)
    .bind(&metadata.mb_track_id)
    .bind(&metadata.mb_album_id)
    .bind(&metadata.mb_artist_id)
    .bind(&metadata.mb_albumartist_id)
    .bind(&metadata.mb_releasetrack_id)
    .bind(&metadata.mb_releasegroup_id)
    .bind(&metadata.trackdisambig)
    .bind(&metadata.album_type)
    /*     .bind(&metadata.albumtypes)
     */
    .bind(&metadata.acoustid_fingerprint)
    .bind(&metadata.acoustid_id)
    .bind(&metadata.asin)
    .bind(&metadata.isrc)
    .bind(&metadata.catalog_num)
    .bind(&metadata.script)
    /*     .bind(&metadata.language)
     */
    .bind(&metadata.country)
    .bind(&metadata.albumstatus)
    .bind(&metadata.media)
    .bind(&metadata.album_disambig)
    .bind(&metadata.release_group_disambig)
    /*     .bind(&metadata.disctitle)
     */
    .bind(&metadata.encodedby)
    .bind(&metadata.original_year)
    .bind(&metadata.initial_key)
    .bind(metadata.bit_rate)
    /*     .bind(&metadata.bitrate)
        .bind(&metadata.bitrate_mode) */
    /*     .bind(&metadata.encoder_info)
     */
    .bind(&metadata.encoder_settings)
    /*     .bind(&metadata.format)
    .bind(&metadata.bitdepth)
    .bind(&metadata.channels) */
    .bind(metadata.track)
    .bind(metadata.disc)
    /*     .bind(&metadata.codec)
     */
    .bind(metadata.length)
    .bind(&metadata.label)
    .bind(metadata.sample_rate)
    .bind(metadata.bit_depth)

    /*     .bind(&metadata.sample_rate)
    .bind(&metadata.bits_per_sample) */
    .bind(&init_time)
    .bind(&init_time)
    .bind(album_id)
    .bind(false)
    .execute(&mut *tx)
    .await?)
}
