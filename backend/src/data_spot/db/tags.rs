
struct TagDB {
    id : i64,
    name : String,
}

impl TagDB {
    pub fn new(id: i64, name: String) -> Self {
        Self {
            id,
            name,
        }
    }
}

pub async fn add_tag(
    db: &sqlx::SqlitePool,
    tag: &TagDB
) -> Result<i64, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO tags (name) VALUES (?) RETURNING id",
        tag.name
    )
    .fetch_one(db)
    .await?
    .id
    .ok_or(sqlx::Error::RowNotFound)
}

pub async fn get_tag(
    db: &sqlx::SqlitePool,
    id: i64
) -> Result<Option<TagDB>, sqlx::Error> {
    let tag = sqlx::query!(
        "SELECT id, name FROM tags WHERE id = ?",
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(tag.map(|record| TagDB {
        id: record.id,
        name: record.name,
    }))
}
pub async fn delete_tag(
    db: &sqlx::SqlitePool,
    id: i64
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM tags WHERE id = ?",
        id
    )
    .execute(db)
    .await?;

    Ok(())
}
pub async fn get_tags(
    db: &sqlx::SqlitePool,
) -> Result<Vec<TagDB>, sqlx::Error> {
    let tags = sqlx::query!(
        "SELECT id, name FROM tags"
    )
    .fetch_all(db)
    .await?;

    Ok(tags.into_iter().map(|record| TagDB {
        id: record.id,
        name: record.name,
    }).collect())
}
