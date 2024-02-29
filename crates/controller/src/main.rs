use reqwest_eventsource::{Event as SseEvent, EventSource};
use tokio_stream::StreamExt;
use woofer::Event;

#[tokio::main]
async fn main() {
    let mut es = EventSource::get("http://localhost:8080/state");
    while let Some(event) = es.next().await {
        match event {
            Ok(SseEvent::Open) => println!("Connection Open!"),
            Ok(SseEvent::Message(message)) => {
                let event: Event = serde_json::from_str(&message.data).unwrap();
                dbg!(event);
            }
            Err(err) => {
                println!("Error: {}", err);
                es.close();
            }
        }
    }
}
