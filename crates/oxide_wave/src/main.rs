use data_dust::types::msg::SubMsg;
use dotenvy::dotenv;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{watch, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use utils::create_msg;

mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = "127.0.0.1:8081".to_string();

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    println!("Websocket server listening on: {}", addr);
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string())
        .parse::<String>()
        .expect("REDIS_URL must be set!");
    let redis_client = redis::Client::open(redis_url).expect("Failed to connect to Redis");

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

            match serde_json::from_str::<SubMsg>(msg) {
                Ok(sub_msg) => match sub_msg.message_type.as_str() {
                    "subscribe" => {
                        for channel in &sub_msg.channels {
                            let ws_sender_clone = Arc::clone(&ws_sender);
                            let client_clone = client.clone();
                            let subscriptions_clone = Arc::clone(&subscriptions);
                            let channel_clone = channel.clone();

                            let (tx, rx) = watch::channel(false);
                            subscriptions_clone
                                .lock()
                                .await
                                .insert(channel_clone.clone(), tx);

                            tokio::spawn(async move {
                                let pub_conn = client_clone
                                    .get_async_connection()
                                    .await
                                    .expect("Failed to connect to Redis for pubsub");
                                let mut pubsub = pub_conn.into_pubsub();
                                pubsub
                                    .subscribe(&channel_clone)
                                    .await
                                    .expect("Failed to subscribe to channel");

                                let mut message_stream = pubsub.on_message();
                                let mut rx = rx;

                                loop {
                                    tokio::select! {
                                        msg = message_stream.next() => {
                                            if let Some(msg) = msg {
                                                let payload: String =
                                                    msg.get_payload().expect("Failed to get payload");

                                                let res = create_msg(
                                                    "data",
                                                    &vec![channel_clone.clone()],
                                                    Some(&payload),
                                                );

                                                let mut sender = ws_sender_clone.lock().await;
                                                sender
                                                    .send(Message::Text(res))
                                                    .await
                                                    .expect("Error sending message");
                                            }
                                        }
                                        _ = rx.changed() => {
                                            break;
                                        }
                                    }
                                }
                                let unsub_conn = client_clone
                                    .get_async_connection()
                                    .await
                                    .expect("Failed to connect to Redis for unsubscribe");
                                let mut unsub_pubsub = unsub_conn.into_pubsub();
                                unsub_pubsub
                                    .unsubscribe(&channel_clone)
                                    .await
                                    .expect("Failed to unsubscribe");
                            });
                        }

                        let res = create_msg("subscribed", &sub_msg.channels, Some("Subscribed"));
                        ws_sender
                            .lock()
                            .await
                            .send(Message::Text(res))
                            .await
                            .expect("Error sending message");
                    }

                    "unsubscribe" => {
                        for channel in &sub_msg.channels {
                            let mut subs = subscriptions.lock().await;
                            if let Some(tx) = subs.remove(channel) {
                                tx.send(true).expect("Failed to send unsubscribe signal");
                            }
                        }

                        let res =
                            create_msg("unsubscribed", &sub_msg.channels, Some("Unsubscribed"));
                        ws_sender
                            .lock()
                            .await
                            .send(Message::Text(res))
                            .await
                            .expect("Error sending message");
                    }

                    _ => {
                        let res = create_msg("error", &Vec::new(), Some("Invalid message type"));
                        ws_sender
                            .lock()
                            .await
                            .send(Message::Text(res))
                            .await
                            .expect("Error sending message");
                    }
                },
                Err(e) => {
                    println!("Error parsing JSON: {}", e);
                    let error_msg =
                        create_msg("error", &Vec::new(), Some(&format!("Invalid JSON: {}", e)));
                    ws_sender
                        .lock()
                        .await
                        .send(Message::Text(error_msg))
                        .await
                        .expect("Error sending error message");
                }
            }
        }
    }
    println!("WebSocket connection closed: {}", addr);
}
