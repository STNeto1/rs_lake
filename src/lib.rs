use anyhow::Result;
use scylla::{transport::errors::NewSessionError, Session, SessionBuilder};

pub async fn create_scylla_client(uri: &String) -> Result<Session, NewSessionError> {
    println!("connecting to {}", uri);

    let session = SessionBuilder::new().known_node(uri).build().await?;
    return Ok(session);
}
