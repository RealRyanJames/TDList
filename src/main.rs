use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io::stdin;

use crate::info::Connect;
mod connection;
mod data;
mod info;
mod sql;
mod todos;

#[tokio::main]
async fn main() -> Result<(), dotenvy::Error> {
    let point_message = info::MessageOut {
        name: String::from(""),
        message: String::from("No Data from Env File Found"),
    };

    let mut todo_1 = String::new();
    stdin().read_line(&mut todo_1).unwrap();

    let todos = todo_1.split_whitespace().collect::<Vec<_>>();

    println!("{}", format!("{}", "Enter First Todo: ".to_uppercase()));
    let t1 = todos[0].parse::<String>();
    println!("{}", format!("{}", "Enter Second Todo: ".to_uppercase()));
    let t2 = todos[1].parse::<String>();
    println!("{}", format!("{}", "Enter Third Todo: ".to_uppercase()));
    let t3 = todos[2].parse::<String>();

    let mut t: Vec<String> = Vec::new();
    t.push(format!("Todo 1: {:?}", t1));
    t.push(format!("Todo 2: {:?}", t2));
    t.push(format!("Todo 3: {:?}", t3));

    let todos = todos::todos(t);

    let mut index = 0;

    for el in todos.iter() {
        println!("{}", format!("Todo: {} Index: {}", el.to_string(), index));
        index += 1;
    }

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
