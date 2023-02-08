pub mod queries {
    pub static ADD_LOG_QUERY: &str = r#"
    INSERT INTO ks.logs (topic, timestamp, data, type)
    VALUES (?, ?, ?, ?);
  "#;

    pub static SELECT_LOGS_FROM_QUERY: &str = r#"
        SELECT *
        FROM ks.logs
        WHERE topic = ?;
    "#;
}
