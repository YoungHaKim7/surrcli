use serde::Deserialize;

/// Main database connection configuration
#[derive(Debug, Clone)]
pub struct SurrDB {
    pub host: String,
    pub namespace: String,
    pub database: String,
    pub user: String,
    pub pass: String,
    pub schema: String,
    pub pretty: bool,
    pub timeout: u64,
    pub comple: usize,
    pub query: String,
}

impl Default for SurrDB {
    fn default() -> Self {
        Self {
            host: "0.0.0.0:80".to_string(),
            namespace: "surr".to_string(),
            database: "surr".to_string(),
            user: "root".to_string(),
            pass: String::new(),
            schema: "http".to_string(),
            pretty: true,
            timeout: 5,
            comple: 5,
            query: String::new(),
        }
    }
}

/// Response payload from SurrealDB
#[derive(Debug, Deserialize)]
pub struct Payload {
    pub time: String,
    pub status: String,
    pub result: serde_json::Value,
}

/// Profile stored in SQLite
#[derive(Debug, Clone)]
pub struct Profile {
    pub pid: i32,
    pub idx: String,
    pub host: String,
    pub sch: String,
    pub dbuser: String,
    pub ns: String,
    pub db: String,
    pub date: String,
}

/// Saved query stored in SQLite
#[derive(Debug, Clone)]
pub struct SavedQuery {
    pub qid: i32,
    pub idx: String,
    pub query: String,
}
