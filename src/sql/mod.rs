pub async fn get_info(pool: &sqlx::PgPool, name: String) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO me (name) VALUES ($1)")
        .bind(name)
        .execute(pool)
        .await?;

    Ok(())
}

// pub async fn get_user(pool: &sqlx::PgPool, id: i32) -> Result<(), sqlx::Error> {
//     let s = sqlx::query_as("SELECT * FROM Person WHERE id = $1")
//         .bind(id)
//         .fetch_one(pool)
//         .await?;
//
//     Ok(s)
// }
