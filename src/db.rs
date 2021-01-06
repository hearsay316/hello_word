use crate::models::{TodoList, TodoItem};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;
pub async fn get_todo_s(client:&Client) ->Result<Vec<TodoList>,io::Error>{
    let statement = client
        .prepare("select * from todo_list order by id desc")
        .await.unwrap();

    let todos = client
        .query(&statement, &[])
        .await.unwrap()
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}
pub async fn get_item(client:&Client,list_id:i32)->Result<Vec<TodoItem>,io::Error>{
    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id")
        .await.unwrap();

    let items = client
        .query(&statement, &[&list_id])
        .await
        .expect("error getting todo lists")

        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(items)
}