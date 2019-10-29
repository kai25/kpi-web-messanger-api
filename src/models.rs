use tokio_postgres::{ Row, Error as DBError };
use crate::db::DBService;
use futures::Future;
use async_trait::async_trait;
use std::fmt::{Display, Formatter, Error as FmtError};

use serde::{Serialize};

pub trait Model<T> {
    fn from_row(row: &Row) -> T;
}

#[async_trait]
pub trait DAO<T> {
    async fn get_by_id(db: &DBService, id: i32) -> Result<Option<T>, DBError>;
    async fn create(db: &DBService, obj: &T) -> Result<i32, DBError>;
    async fn get_all(db: &DBService) -> Result<Vec<T>, DBError>;
}


#[derive(Serialize)]
pub struct Message {
    pub id: i32,
    pub text: String,
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "Message (id: {}, text: {})", self.id, self.text)
    }
}

impl Model<Message> for Message {
    fn from_row(row: &Row) -> Message {
        Message {
            id: row.get("id"),
            text: row.get("text"),
        }
    }
}

pub struct MessageDAO {}

#[async_trait]
impl DAO<Message> for MessageDAO {
    async fn get_by_id(db: &DBService, id: i32) -> Result<Option<Message>, DBError> {
        db.client
            .query("SELECT * FROM messages WHERE id = 0", &[&id]).await
            .map(|rows| {
                match rows.len() {
                    0 => None,
                    1 => Some(Message::from_row(&rows[0])),
                    _ => panic!("There should be common DBError. Multiple records for id"),
                }
            })
    }

    async fn create(db: &DBService, obj: &Message) -> Result<i32, DBError> {
        db.client
            .query_one("INSERT INTO messages(text) VALUES ($1) RETURNING id", &[&obj.text])
            .await
            .map(|row| row.get(0))
    }

    async fn get_all(db: &DBService) -> Result<Vec<Message>, DBError> {
        db.client
            .query("SELECT * FROM messages", &[])
            .await
            .map(|rows| rows.into_iter()
                    .map(|row| Message::from_row(&row)).collect())
    }
}

