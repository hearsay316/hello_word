use crate::models::{TodoList};
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