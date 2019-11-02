use crate::db::DBService;
use crate::redis_service::RedisService;

pub struct ServiceRegistry {
    pub db: DBService,
    pub redis: RedisService,
}