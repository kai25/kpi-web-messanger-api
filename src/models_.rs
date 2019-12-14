use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::{Error as DBError, Row};

use crate::db_service::DBService;
use crate::permissions::Role;
use crate::date_utils::{parse_date, date_to_str};

use serde::{Deserialize, Serialize};

pub trait Model<T> {
    fn from_row(row: &Row) -> T;
}

#[async_trait]
pub trait DAO<T> {
    async fn get_by_id(db: &DBService, id: i32) -> Result<Option<T>, DBError>;
    async fn create(db: &DBService, obj: &T) -> Result<i32, DBError>;
    async fn update(db: &DBService, obj: &T) -> Result<(), DBError>;
    async fn get_all(db: &DBService) -> Result<Vec<T>, DBError>;
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub role: String,
}

impl User {
    pub fn get_role(&self) -> Role {
        Role::parse(&self.role).unwrap()
    }
}

impl Model<User> for User {
    fn from_row(row: &Row) -> User {
        User {
            id: row.get("id"),
            name: row.get("name"),
            password_hash: row.get("password_hash"),
            created_at: DateTime::from_str(row.get("created_at")).unwrap(),
            role: row.get("role"),
        }
    }
}

pub struct UserDAO {}

#[async_trait]
impl DAO<User> for UserDAO {
    async fn get_by_id(db: &DBService, id: i32) -> Result<Option<User>, DBError> {
        db.client
            .query("SELECT * FROM users WHERE id = $1", &[&id])
            .await
            .map(|rows| match rows.len() {
                0 => None,
                1 => Some(User::from_row(&rows[0])),
                _ => panic!("There should be common DBError. Multiple records for id"),
            })
    }

    async fn create(db: &DBService, obj: &User) -> Result<i32, DBError> {
        db.client
            .query_one(
                "INSERT INTO users(name, password_hash, created_at, role) VALUES ($1, '$2', '$3', '$4' ) RETURNING id",
                &[&obj.name, &obj.password_hash, &obj.created_at.to_string(), &obj.role.to_string()],
            )
            .await
            .map(|row| row.get(0))
    }

    async fn update(db: &DBService, obj: &User) -> Result<(), DBError> {
        db.client
            .query_one(
                "UPDATE users set name = '$1', password_hash = '$2', role = '$3' where id = $4",
                &[
                    &obj.name,
                    &obj.password_hash,
                    &obj.role.to_string(),
                    &obj.id,
                ],
            )
            .await
            .map(|row| ())
    }

    async fn get_all(db: &DBService) -> Result<Vec<User>, DBError> {
        db.client
            .query("SELECT * FROM users", &[])
            .await
            .map(|rows| rows.into_iter().map(|row| User::from_row(&row)).collect())
    }
}

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
