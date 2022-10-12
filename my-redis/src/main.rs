use mini_redis::{client, Result};

async fn test() {
    println!("Test")
}

#[tokio::main]
async fn main() -> Result<()> {

    let test_future = test();

    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    test_future.await;

    Ok(())
}