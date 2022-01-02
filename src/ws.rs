#[allow(dead_code)]
const API_VERSION: u8 = 9;

use std::sync::{Arc, Mutex};
// use tokio::sync::Arc;

// use std::env;
use url;
// use std::io::stdout;
use futures_util::{/* pin_mut, */ SinkExt, StreamExt};

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// use crate::ws;

// env::var("DISCORD_TOKEN").unwrap()

pub async fn main1() {
    let url = url::Url::parse(
        // format!("wss://gateway.discord.gg/?v={}&encoding=json", API_VERSION).as_str(),
        "ws://localhost:8080",
    )
    .unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut s, mut read) = ws_stream.split();

    tokio::spawn(async move { s.send(Message::Text("skjskj".to_string())).await })
        .await
        .unwrap()
        .unwrap();

    let message = read.next().await.unwrap().unwrap();

    if let Message::Text(txt) = message {
        println!("{}", txt);

        assert_eq!(txt, String::from("skjskj"));
    }

    // ww.lock().unwrap().close(None).await;
    // })
    // .await
    // .unwrap();
}
