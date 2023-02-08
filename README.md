## Generic Data Lake



### Instructions

```sh
# run docker with infra
$ docker compose up -d

# test connection and configure scylla ks and table
$ cargo run --bin scylla

# run web server
$ cargo run --bin rs_lake
```

---

- Features
  - Tokio + Axum
  - Scylla db
  - Can store any valid json on any topic
  - Can fetch from any topic
