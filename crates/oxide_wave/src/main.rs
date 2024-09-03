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

            match serde_json::from_str::<SubMsg>(msg) {
                Ok(msg) => match msg.message_type.as_str() {
                    "subscribe" => {
                        let ws_sender_clone = Arc::clone(&ws_sender);
                        let channel = msg.channel.clone();
                        let client_clone = client.clone();
                        let subscriptions_clone = Arc::clone(&subscriptions);

                        let (tx, mut rx) = watch::channel(false);
                        subscriptions_clone.lock().await.insert(channel.clone(), tx);

                        tokio::spawn(async move {
                            let pub_conn = client_clone
                                .get_async_connection()
                                .await
                                .expect("Failed to connect to Redis for pubsub");
                            let mut pubsub = pub_conn.into_pubsub();
                            pubsub
                                .subscribe(&channel)
                                .await
                                .expect("Failed to subscribe to channel");

                            let task_pub_conn = client_clone
                                .get_async_connection()
                                .await
                                .expect("Failed to connect to Redis for pubsub");
                            let mut task_pubsub = task_pub_conn.into_pubsub();

                            let mut message_stream = pubsub.on_message();

                            loop {
                                tokio::select! {
                                    msg = message_stream.next() => {
                                        if let Some(msg) = msg {

                                            let payload: String =
                                                msg.get_payload().expect("Failed to get payload");

                                            let res = create_msg(
                                                "data",
                                                &channel,
                                                Some(&payload.to_string()),
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
                            task_pubsub
                                .unsubscribe(&channel)
                                .await
                                .expect("Failed to unsubscribe");
                        });

                        let res = create_msg("subscribed", &msg.channel, Some("Subscribed"));
                        ws_sender
                            .lock()
                            .await
                            .send(Message::Text(res))
                            .await
                            .expect("Error sending message");
                    }
                    "unsubscribe" => {
                        let mut subs = subscriptions.lock().await;
                        if let Some(tx) = subs.remove(&msg.channel) {
                            tx.send(true).expect("Failed to send unsubscribe signal");
                        }

                        let res = create_msg("unsubscribed", &msg.channel, Some("Queued"));
                        ws_sender
                            .lock()
                            .await
                            .send(Message::Text(res))
                            .await
                            .expect("Error sending message");
                    }
                    _ => {
                        let res = create_msg("error", &msg.channel, Some("Invalid message type"));
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
                        create_msg("error", "system", Some(&format!("Invalid JSON: {}", e)));
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
