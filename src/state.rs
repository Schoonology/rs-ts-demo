use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub message: String,
    pub timestamp: i64,
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub messages: Arc<Mutex<Vec<Post>>>,
    pub updates: broadcast::Sender<Post>,

    // We keep this Receiver around to keep the Sender from closing.
    // We wrap it in `Arc` to provide a naïve, no-copy Clone implementation.
    // We don't publicize it so that route handlers don't actually use it.
    #[allow(unused)]
    update_vent: Arc<broadcast::Receiver<Post>>,
}

impl AppState {
    pub fn new() -> Self {
        let (sender, receiver) = broadcast::channel(32);

        AppState {
            messages: Arc::new(Mutex::new(Vec::new())),
            updates: sender,
            update_vent: Arc::new(receiver),
        }
    }
}
