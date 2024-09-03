use data_dust::types::msg::SubMsg;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{watch, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use utils::create_msg;

mod utils;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    println!("Websocket server listening on: {}", addr);
    let redis_client =
        redis::Client::open("redis://127.0.0.1:6379/").expect("Failed to connect to Redis");

    while let Ok((stream, _)) = listener.accept().await {
        let client_clone = redis_client.clone();
        tokio::spawn(handle_connection(stream, client_clone));
    }
}

async fn handle_connection(stream: TcpStream, client: redis::Client) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = accept_async(stream)
        .await
        .expect("Error during websocket handshake");
    println!("New Websocket connection established: {}", addr);

    let (ws_sender, mut ws_receiver) = ws_stream.split();
    let ws_sender = Arc::new(Mutex::new(ws_sender));

    let mut _conn = client
        .get_async_connection()
        .await
        .expect("Failed to get Redis connection");

    let pub_conn = client
        .get_async_connection()
        .await
        .expect("Failed to connect to Redis for pubsub");
    let pubsub = pub_conn.into_pubsub();
    let _pubsub = Arc::new(Mutex::new(pubsub));

    let subscriptions: Arc<Mutex<HashMap<String, watch::Sender<bool>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg.expect("Error reading message");
        if msg.is_text() || msg.is_binary() {
            let msg = msg.to_text().expect("Error converting message to text");

        //     match serde_json::from_str::<SubMsg>(msg) {
        //         Ok(msg) => match msg.message_type.as_str() {

        //         Err(e) => {
        //             println!("Error parsing JSON: {}", e);
        //             let error_msg =
        //                 create_msg("error", "system", Some(&format!("Invalid JSON: {}", e)));
        //             ws_sender
        //                 .lock()
        //                 .await
        //                 .send(Message::Text(error_msg))
        //                 .await
        //                 .expect("Error sending error message");
        //         }
        //     }
        // }
    }
    println!("WebSocket connection closed: {}", addr);
}
