use std::{io::Error as IoError, net::SocketAddr};

// use futures_channel::mpsc::unbounded;
use futures_util::SinkExt;
// use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use futures_util::StreamExt;
// use std::{sync::Arc, sync::Mutex};

use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    println!("WebSocket connection established: {}", addr);

    let (mut outgoing, mut incoming) = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred")
        .split();

    loop {
        let rr = match match incoming.next().await {
            Some(v) => v,
            None => break,
        } {
            Ok(rr) => rr,
            Err(e) => {
                println!("Error: {}", e);

                panic!("ajaj")
            }
        }
        .clone();

        println!("msg: {:?}", rr.to_text());

        outgoing.send(rr).await.unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    println!("Listening on");

    // Let's spawn the handling of each connection in a separate task.
    if let Ok((stream, addr)) = (TcpListener::bind(&"127.0.0.1:8080").await)
        .expect("Failed to bind")
        .accept()
        .await
    {
        tokio::spawn(handle_connection(stream, addr)).await.unwrap();
    }

    Ok(())
}
