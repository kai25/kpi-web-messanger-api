use crate::db_service::DBService;
use crate::redis_service::RedisService;

pub struct ServiceRegistry {
    pub db: DBService,
    pub redis: RedisService,
}
