use crate::models::{TodoList, TodoItem};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client.prepare("SELECT * FROM todo_list ORDER BY id desc").await.unwrap();

    let todos = client.query(&statement, &[])
                    .await
                    .expect("Error getting todo lists")
                    .iter()
                    .map(|row| TodoList::from_row_ref(row).unwrap())
                    .collect::<Vec<TodoList>>();

    Ok(todos)
 }


 pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {
     let statement = client.prepare("SELECT * FROM todo_item").await.unwrap();

     let todos = client.query(&statement, &[&list_id])
                        .await
                        .expect("Error getting todo items")
                        .iter()
                        .map(|row| TodoItem::from_row_ref(row).unwrap())
                        .collect::<Vec<TodoItem>>();

      Ok(todos)                  
 }


 pub async fn create_todo(client: &Client, title: String ) -> Result<TodoList, io::Error> {
    let statement = client.prepare("INSERT INTO todo_list (title) VALUES ($1) returning id, title").await.unwrap();
    
    client.query(&statement, &[&title])
            .await
            .expect("Error creating todo list")
            .iter()
            .map(|row| TodoList::from_row_ref(row).unwrap())
            .collect::<Vec<TodoList>>()
            .pop()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Error creting todo list"))
 }