use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // establish the connection to mini-redis
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set key: "hello" and value: "world"
    client.set("hello", "world".into()).await?;

    // get the value when "key=hello"
    let result = client.get("hello").await?;

    println!("get from the client side={:?}", result);

    Ok(())
}

