use crate::models::{TodoList, TodoItem};
use deadpool_postgres::{Client};
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
pub async fn get_items(client:&Client,list_id:i32)->Result<Vec<TodoItem>,io::Error>{
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
pub async fn create_todo(client:&Client,title:String)->Result<TodoList,io::Error>{
    let statement = client
        .prepare("insert into todo_list (title) values ($1) returning id,title")
        .await.unwrap();
      client
          .query(&statement,&[&title])
        .await
        .expect("纷纷")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
          .pop()
          .ok_or(io::Error::new(io::ErrorKind::Other,"错误二"))
}
pub async fn check_item(client:&Client,list_id:i32,item_id:i32)->Result<(),io::Error>{
    let statement = client
        //update todo_item set checked = true where list_id = $1 and id = $2 and checked = false
        .prepare("update todo_item set checked = true where list_id = $1 and id = $2 and checked = false")
        .await.unwrap();
    let result = client.execute(&statement,&[&list_id,&item_id]).await.expect("Error ch");
    match result {
        ref updated if *updated ==1=>Ok(()),
        _=>Err(io::Error::new(io::ErrorKind::Other,"数据库读写错误了"))
}}