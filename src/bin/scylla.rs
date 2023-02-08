use anyhow::{Error, Ok, Result};
use rs_lake::create_scylla_client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let uri = "127.0.0.1:9042".to_owned();
    let client = create_scylla_client(&uri).await?;

    println!("connected to scylla =)");

    client.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}", &[]).await?;
    client.query("CREATE TABLE IF NOT EXISTS ks.logs (topic TEXT, time TIMESTAMP, type VARCHAR, data TEXT, PRIMARY KEY (topic, time))", &[]).await?;

    return Ok(());
}
