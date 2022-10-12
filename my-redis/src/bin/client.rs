use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        responder: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        responder: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    // tx: transmitter
    // rx: receiver
    // obviously...
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        let mut connection = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, responder } => {
                    let result = connection.get(&key).await;

                    // Explicitly ignore errors:
                    // If the requesting task is no longer interested, the channel will be closed
                    let _ = responder.send(result);
                }
                Set {
                    key,
                    value,
                    responder,
                } => {
                    let result = connection.set(&key, value).await;

                    // Explicitly ignore errors:
                    // If the requesting task is no longer interested, the channel will be closed
                    let _ = responder.send(result);
                }
            }
        }
    });

    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: String::from("test"),
            responder: resp_tx,
        };
        tx.send(cmd).await.unwrap();

        let response = resp_rx.await;
        println!("Got: {:?}", response);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: String::from("test"),
            value: "test".into(),
            responder: resp_tx,
        };
        tx2.send(cmd).await.unwrap();

        let response = resp_rx.await;
        println!("Got: {:?}", response);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap()
}
