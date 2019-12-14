use async_trait::async_trait;
use tokio_postgres::{Error as DBError, Row};

use crate::db_service::DBService;


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
