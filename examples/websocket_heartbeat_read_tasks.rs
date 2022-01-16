use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio::time;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

static WS_URL: &str = "wss://";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(32);
    let tx_loop = tx.clone();

    // Task to receive websocket messages for processing
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Receive task: got message {}", msg);
        }
    });

    // In a loop connect to websocket and spawn tasks for reading data from and for writing pings to websocket
    loop {
        println!("\nLooping...");
        let tx_send = tx_loop.clone();

        let (mut sink, mut stream) = match connect_async(Url::parse(&WS_URL).unwrap()).await {
            Ok((stream, response)) => {
                println!("{:?}", response);
                stream.split()
            }
            Err(e) => panic!("Websocket: could not connect. {}", e),
        };

        tokio::select! {
            _ = async {
                let send_task = tokio::spawn(async move {
                    loop {
                        time::sleep(time::Duration::from_secs(10)).await;
                        match sink.send(Message::Text("PING".to_string())).await {
                            Ok(_) => println!("Send task: Sent message"),
                            Err(_) => panic!("Send task: Could not send message"),
                        }
                    }
                });
                send_task.await.unwrap();
            } => {}

            _ = async {
                let read_task = tokio::spawn(async move {
                    while let Some(message) = stream.next().await {
                        match message {
                            Ok(msg) => {
                                match msg {
                                    Message::Text(msg) => {
                                        println!("Read task: received message (type: Text): {}", msg);

                                        if tx_send.send(msg).await.is_err() {
                                            panic!("Read task: Could not send to channel");
                                        };
                                    }
                                    Message::Binary(_) | Message::Ping(_) | Message::Pong(_) => (),
                                    Message::Close(e) => {
                                        println!("Read task: Received message (type: Close): {:?}", e.unwrap());
                                        break;
                                    }
                                };
                            }
                            Err(e) => eprintln!("Read task: Error: {}", e),
                        }
                    }
                });
                read_task.await.unwrap();
            } => {}
        }
    }
}
