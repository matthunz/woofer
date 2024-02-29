use axum::{
    http::StatusCode,
    response::{sse, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::{stream, Stream, TryStreamExt};
use std::time::Duration;
use tokio_stream::StreamExt;
use woofer::{Event, Message};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/state", get(sse_handler))
        .route("/state", post(set_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn set_state(Json(msg): Json<Message>) -> StatusCode {
    match msg {
        Message::Pose { body } => {
            dbg!(body);
        }
    }

    StatusCode::OK
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<sse::Event, axum::Error>>> {
    let stream = stream::repeat_with(|| sse::Event::default().json_data(Event::default()))
        .map_err(Into::into)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
