use std::convert::Infallible;

use axum::{
    extract::State,
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive},
        Html, IntoResponse, Response, Sse,
    },
    routing::get,
    Json, Router,
};
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};
use tower_http::services::ServeDir;

use crate::{
    errors::Result,
    state::{AppState, Post},
};

/**
 * Returns an axum-compatible Router for the HTTP API.
 */
pub fn create(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/updates", get(stream_updates))
        .route("/posts", get(fetch_posts).post(append_post))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state)
}

/**
 * Renders and returns the index HTML.
 */
async fn index() -> impl IntoResponse {
    Html(include_str!("./frontend/index.html"))
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
    State(AppState {
        messages, updates, ..
    }): State<AppState>,
    Json(post): Json<Post>,
) -> Result<impl IntoResponse> {
    messages.lock().await.push(post.clone());

    updates.send(post)?;

    Ok((StatusCode::NO_CONTENT, ""))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{HeaderValue, Request, Response, StatusCode},
        Router,
    };
    use serde_json::json;
    use tower::{Service, ServiceExt};

    /**
     * Creates a Router for testing.
     */
    fn subject() -> Router {
        super::create(crate::state::AppState::new())
    }

    /**
     * Reads collects the HTTP entity from `response` into a String.
     *
     * Panics on _any_ error.
     */
    async fn body_from_response(response: Response<Body>) -> String {
        use http_body_util::BodyExt as _;

        String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .into(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_empty_get() {
        let response = subject()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/posts")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_from_response(response).await, "[]");
    }

    #[tokio::test]
    async fn test_read_after_write() {
        let mut router = subject().into_service();
        let response = router
            .call(
                Request::builder()
                    .method("POST")
                    .uri("/posts")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        json!({
                            "message": "This is a test.",
                            "timestamp": 42
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        assert_eq!(body_from_response(response).await, "");

        let response = router
            .call(
                Request::builder()
                    .method("GET")
                    .uri("/posts")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            body_from_response(response).await,
            "[{\"message\":\"This is a test.\",\"timestamp\":42}]"
        );
    }

    #[tokio::test]
    async fn test_sse_headers() {
        let response = subject()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/updates")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type"),
            Some(&HeaderValue::from_static("text/event-stream"))
        );
    }

    #[tokio::test]
    async fn test_index() {
        let response = subject()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type"),
            Some(&HeaderValue::from_static("text/html; charset=utf-8"))
        );
    }
}
