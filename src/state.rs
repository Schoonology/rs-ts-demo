use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub messages: Arc<Mutex<Vec<Post>>>,
    pub updates: broadcast::Sender<Post>,
}

impl AppState {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(32);

        AppState {
            messages: Arc::new(Mutex::new(Vec::new())),
            updates: sender,
        }
    }
}
