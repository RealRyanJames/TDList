// use sqlx::postgres::PgPoolOptions;

pub async fn get_info(
    pool: &sqlx::PgPool,
    name: &str,
    todos: Vec<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO Todos (name, todos Values ($1, $2)")
        .bind(name.to_string())
        .bind(todos)
        .execute(pool)
        .await?;

    Ok(())
}
