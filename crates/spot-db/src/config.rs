use std::env::var;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Option<String>,
    pub dbname: String,
    pub max_pool_size: usize,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenvy::dotenv().ok();

        Ok(Self {
            host: var("DATABASE_HOST")?,
            port: var("DATABASE_PORT").unwrap().parse()?,
            user: var("DATABASE_USER")?,
            password: var("DATABASE_PASSWORD").ok().filter(|p| !p.is_empty()),
            dbname: var("DATABASE_DBNAME")?,
            max_pool_size: var("DATABASE_MAX_POOL_SIZE")?.parse()?,
        })
    }
}
