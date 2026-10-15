use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::info::Connect;
mod sql;
mod connection;
mod data;
mod info;

#[tokio::main]
async fn main() -> Result<(), dotenvy::Error> {
    let point_message = info::MessageOut {
        name: String::from(""),
        message: String::from("No Data from Env File Found"),
    };

    let connect = info::ConnectionSuccess::Connected();

    let db_name = env::var("DB").expect(&point_message.message);

    let connection_data: connection::ConnectionSpace = connection::ConnectionSpace {
        db_name: String::from(db_name),
    };

    dotenv().ok();

    let db_url = env::var("DB_USER_URL").expect(&point_message.message);
    PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("No Connection Found");

    println!(
        "{}",
        format!("{}", "Connection Successfully".to_uppercase())
    );

    if connect.is_connected {
        let message = info::MessageOut {
            message: "Your Connected".to_uppercase(),
            name: "Allowed",
        };

        println!(
            "{}",
            format!(
                "{} {} | {}",
                message.name.to_string(),
                message.message,
                connection_data.db_name.to_uppercase()
            )
        )
    }

    Ok(())
}
