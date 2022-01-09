use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

static WS_URL: &str = "wss://";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(&WS_URL).unwrap();

    loop {
        println!("\nLooping..");

        let (ws, resp) = connect_async(&url)
            .await
            .expect("Websocket error. Could not connect.");

        println!("{:?}", resp);

        let (mut sink, mut stream) = ws.split();

        // Task for reading stream
        let ws_read_task = tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                match message {
                    Ok(msg) => {
                        match msg {
                            Message::Text(msg) => {
                                println!("Received message (type: Text): {}", msg)
                            }
                            Message::Ping(_) => println!("Received message (type: Ping)"),
                            Message::Pong(_) => println!("Received message (type: Pong)"),
                            Message::Binary(_) => println!("Received message (type: Binary)"),
                            Message::Close(e) => {
                                println!("Received message (type: Close): {:?}", e.unwrap())
                            }
                        };
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        });

        // Task for sending message
        let ws_send_task = tokio::spawn(async move {
            match sink
                .send(Message::Text(String::from("Message to send")))
                .await
            {
                Ok(_) => println!("Sent message"),
                Err(e) => eprintln!("Error sending message: {}", e),
            }
        });

        ws_read_task.await.unwrap();
        ws_send_task.await.unwrap();
    }
}
