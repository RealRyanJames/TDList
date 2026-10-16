use sqlx::postgres::PgPoolOptions;

use crate::{info::Connect, trait_functions::UIShownWindow};

use std::io::stdin;
mod data;
mod info;
mod sql;
mod trait_functions;
mod ui;

#[derive(Default)]
struct TodoGetter {
    pub id: i32,
    pub data: String,
    pub todos: Option<Vec<String>>,
}

trait AllTodosElements {
    fn get_all(&mut self) -> Self;
}

impl AllTodosElements for TodoGetter {
    fn get_all(&mut self) -> Self {
        let todo_getter = TodoGetter {
            todos: TodoGetter::default().todos,
            data: self.data.to_string(),
            id: self.id,
        };

        todo_getter
    }
}

unsafe extern "C" fn get_x(mut args: ...) -> i32 {
    let x = unsafe { args.next_arg::<i32>() as i32 };
    x
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let x = unsafe { (get_x(20) * 2 * 2) / 2 };

    let mut pos_lines = trait_functions::utils::UIElementsShow {
        xpos: x as f32,
        created: String::from("-"),
    };

    let url = std::env::var("DATABASE_URL").expect("No Input Found");
    let p = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("No DB Found");

    println!("{}", "Connected".to_uppercase());

    let mut todo_1 = String::new();

    for _ in 2..pos_lines.xpos as i32 {
        print!("{}", format!("{:?}", pos_lines.show_ui_screen().created));
    }

    println!("{}", "\n");
    println!("{}", format!("{}\n", "Enter First Todo: ".to_uppercase()));
    let _ = stdin().read_line(&mut todo_1);

    let mut t: Vec<String> = Vec::new();
    t.push(format!("Todo 1: {:?}", todo_1));
    let mut todo_valid = TodoGetter {
        id: 11,
        todos: None,
        data: todo_1.to_string(),
    };

    println!("{}", format!("Todo: {}", todo_valid.get_all().data));

    let _ = sql::get_info(&p, todo_valid.get_all().data).await;

    println!("{}", "Inserted Info to PostgresSQL");

    for _ in 2..pos_lines.xpos as i32 {
        print!("{}", format!("{}", pos_lines.created));
    }

    let connect = info::Success::connected();

    println!(
        "{}",
        format!("{}", "Connection Successfully".to_uppercase())
    );

    if connect.is_connected {
        let message = info::MessageOut {
            message: "Your Connected".to_uppercase(),
            name: "Allowed".to_string(),
        };

        println!(
            "{}",
            format!("{} {}", message.name.to_string(), message.message)
        )
    }

    Ok(())
}
