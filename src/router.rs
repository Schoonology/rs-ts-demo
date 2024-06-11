use std::convert::Infallible;

use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Response, Sse,
    },
    routing::get,
    Json, Router,
};
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};

use crate::{
    errors::Result,
    state::{AppState, Post},
};

pub fn create(state: AppState) -> Router {
    Router::new()
        .route("/updates", get(stream_updates))
        .route("/posts", get(fetch_posts).post(append_post))
        .with_state(state)
}

/**
 * Returns a Server-Sent Events response from all the broadcast updates.
 */
async fn stream_updates(
    State(AppState { updates, .. }): State<AppState>,
) -> Sse<impl Stream<Item = std::result::Result<Event, Infallible>>> {
    let stream = BroadcastStream::new(updates.subscribe()).map(|post| {
        let post = post.expect("Broadcast error.");

        Ok(Event::default().data(serde_json::to_string(&post).expect("Invalid post.")))
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/**
 * Pushes a new post into storage, as well as broadcasting the update.
 */
async fn fetch_posts(
    State(AppState { messages, .. }): State<AppState>,
) -> Result<impl IntoResponse> {
    let guard = messages.lock().await;
    let messages: &Vec<Post> = guard.as_ref();

    // Manually calling `to_string` means we can serialize these messages
    // without copying these messages.
    let json = serde_json::to_string(messages)?;

    // We would use Json here, but there isn't a no-copy option.
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(json)?)
}

/**
 * Pushes a new post into storage, as well as broadcasting the update.
 */
async fn append_post(
    State(AppState { messages, updates }): State<AppState>,
    Json(post): Json<Post>,
) -> Result<impl IntoResponse> {
    messages.lock().await.push(post.clone());

    updates.send(post)?;

    Ok(())
}
