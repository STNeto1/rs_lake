pub mod queries {
    pub static ADD_LOG_QUERY: &str = r#"
    INSERT INTO ks.logs (topic, timestamp, data, type)
    VALUES (?, ?, ?, ?);
  "#;
}
