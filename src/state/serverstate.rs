use super::{asset_cache::AssetCache, config::ServerConfig};

pub struct ServerState {
    #[cfg(feature = "redis_ratelimit")]
    redis_client: redis::Client,
    http_client: reqwest::Client,
    pub cache: AssetCache,
    pub config: ServerConfig,
}

impl ServerState {
    pub fn new(config_filename: &str) -> Self {
        Self {
            #[cfg(feature = "redis_ratelimit")]
            redis_client: redis::Client::open(
                std::env::var("REDIS_URI").expect("Couldn't find REDIS_URI"),
            )
            .expect("Couldn't connect to redis"),
            http_client: reqwest::ClientBuilder::new()
                .user_agent("My user agent")
                .build()
                .unwrap(),
            cache: AssetCache::new(),
            config: ServerConfig::new(config_filename),
        }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.http_client
    }

    #[cfg(feature = "ratelimit")]
    pub fn redis(&self) -> Result<redis::Connection, crate::errors::Errors> {
        Ok(self.redis_client.get_connection()?)
    }
}
