use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::{Error as DBError, Row};

use crate::db_service::DBService;
use crate::date_utils::{parse_date, date_to_str};

use serde::{Deserialize, Serialize};

use crate::models::base::{Model, DAO};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub text: String,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub created_at: DateTime<Utc>,
}

impl Model<Message> for Message {
    fn from_row(row: &Row) -> Message {
        Message {
            id: row.get("id"),
            text: row.get("text"),
            from_user_id: row.get("from_user_id"),
            to_user_id: row.get("to_user_id"),
            created_at: parse_date(row.get("created_at")),
        }
    }
}

pub struct MessageDAO {}

#[async_trait]
impl DAO<Message> for MessageDAO {
    async fn get_by_id(db: &DBService, id: i32) -> Result<Option<Message>, DBError> {
        db.client
            .query("SELECT * FROM messages WHERE id = 0", &[&id])
            .await
            .map(|rows| match rows.len() {
                0 => None,
                1 => Some(Message::from_row(&rows[0])),
                _ => panic!("There should be common DBError. Multiple records for id"),
            })
    }

    async fn create(db: &DBService, obj: &Message) -> Result<i32, DBError> {
        db.client
            .query_one(
                "INSERT INTO messages(text, from_user_id, to_user_id, created_at) VALUES ($1, $2, $3, $4) RETURNING id",
                &[&obj.text, &obj.from_user_id, &obj.to_user_id, &date_to_str(obj.created_at)],
            )
            .await
            .map(|row| row.get(0))
    }

    async fn update(db: &DBService, obj: &Message) -> Result<(), DBError> {
        db.client
            .query_one(
                "INSERT INTO messages(text, from_user_id, to_user_id) VALUES ($2, $3, $4) where id = $1",
                &[&obj.text, &obj.from_user_id, &obj.to_user_id],
            )
            .await
            .map(|_| ())
    }

    async fn get_all(db: &DBService) -> Result<Vec<Message>, DBError> {
        db.client
            .query("SELECT id, text, from_user_id, to_user_id, created_at FROM messages", &[])
            .await
            .map(|rows| {
                rows.into_iter()
                    .map(|row| Message::from_row(&row))
                    .collect()
            })
    }
}
