pub mod queries {
    pub static CREATE_KS: &str = r#"
      CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1};
    "#;

    pub static CREATE_TABLE: &str = r#"
      CREATE TABLE IF NOT EXISTS ks.logs
      (
          topic     TEXT,
          timestamp TIMESTAMP,
          type      VARCHAR,
          data      TEXT,
          PRIMARY KEY (topic, timestamp)
      );
    "#;

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
