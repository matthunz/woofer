use axum::{
    extract::State,
    http::StatusCode,
    response::{sse, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::{stream, Stream, TryStreamExt};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio_stream::StreamExt;
use woofer::{Event, Message};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/state", get(sse_handler))
        .route("/state", post(set_state))
        .with_state(Arc::new(Mutex::new(Event::default())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn set_state(State(state): State<Arc<Mutex<Event>>>, Json(msg): Json<Message>) -> StatusCode {
    match msg {
        Message::Pose { body: _ } => {
            *state.lock().unwrap() = Event::default();
        }
    }

    StatusCode::OK
}

async fn sse_handler(
    State(state): State<Arc<Mutex<Event>>>,
) -> Sse<impl Stream<Item = Result<sse::Event, axum::Error>>> {
    let stream =
        stream::repeat_with(move || sse::Event::default().json_data(state.lock().unwrap().clone()))
            .map_err(Into::into)
            .throttle(Duration::from_millis(100));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
