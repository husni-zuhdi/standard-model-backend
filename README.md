# ⚙️ Standard Model Backend Service

Backend Service for Standard Model

## 👷 How to use?

### 🐥 Development
* Setup PostgreSQL with docker by `docker compose up -d`
* Run it with `cargo run` and then check every API.

## 🔧 Troubleshoots

### 🚨 Diesel missing `libpq` in MacOs | [Solution](https://stackoverflow.com/questions/70313347ld-library-not-found-for-lpq-when-build-rust-in-macos?rq=1)

```bash
# Error Message
error: linking with `cc` failed: exit status: 1
...
...
  = note: ld: library not found for -lpq
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

* Install `libpq` via Hombrew
```bash
brew install libpq && brew link --force libpq
echo 'export PATH="/usr/local/opt/libpq/bin:$PATH"' >> ~/.zshrc
```

* Install Diesel CLI
```bash
cargo install diesel_cli --no-default-features --features postgres
```

* Double check the Diesel CLI
```bash
diesel --version
```

### 🚨 Diesel missing `lmysqlclient` in MacOs | [Solution](https://stackoverflow.com/questions/49569724/installing-the-mysqlclient-library-in-mac-os)

```bash
# Error Message
error: linking with `cc` failed: exit status: 1
...
...
  = note: ld: library not found for -lmysqlclient
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

* Install `lmysqlclient` via Hombrew
```bash
brew install lmysqlclient
```

## 📚 References

* [Build an API in Rust with JWT Authentication](https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/)

* [Practical Clean Architecture in Typescript, Rust & Python ](https://dev.to/msc29/practical-clean-architecture-in-typescript-rust-python-3a6d)

* [Postgres with Docker and Docker compose a step-by-step guide for beginners](https://geshan.com.np/blog/2021/12/docker-postgres/)

* [Connection Strings](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING)

* [Structs Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime/struct.html)

* [Compiler can’t find Diesel’s “table!” macro](https://users.rust-lang.org/t/compiler-cant-find-diesels-table-macro/76699/3)

* [Learn Rust by building a RESTFul API with Actix](https://0xchai.io/blog/restful-api-with-actix)