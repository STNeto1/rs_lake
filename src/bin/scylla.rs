use anyhow::{Error, Ok, Result};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let uri = "127.0.0.1:9042".to_owned();

    let client = rs_lake::create_scylla_client(&uri).await?;

    println!("connected to scylla =)");

    client
        .query(rs_lake::client::queries::CREATE_KS, &[])
        .await?;
    client
        .query(rs_lake::client::queries::CREATE_TABLE, &[])
        .await?;

    return Ok(());
}
