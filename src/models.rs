use tokio_postgres::Row;
use crate::db::DB;

trait Model<T> {
    fn from_row(row: Row) -> T;
}

trait DAO<T> {
    fn get_by_id(&self, &DB)
}

struct Message {
    id: u32,
    text: String,
}

struct MessageDAO {
    fn get_first() -> Message
}

impl Model<Message> for Message {
    fn from_row(row: Row) -> Message {
        Message {
            id: row.get("id"),
            text: row.get("text"),
        }
    }
}
