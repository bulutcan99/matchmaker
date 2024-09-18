use crate::shared::config::config::Config;
use bb8::Pool;
use sidekiq::RedisConnectionManager;

pub async fn connect_redis() -> Option<Pool<RedisConnectionManager>> {
    let config = Config::get();
    if let Some(redis) = &config.queue {
        let manager = RedisConnectionManager::new(redis.uri.clone()).unwrap();
        let redis = Pool::builder().build(manager).await.unwrap();
        Some(redis)
    } else {
        None
    }
}
