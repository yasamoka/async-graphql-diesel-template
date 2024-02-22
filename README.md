## Linux

### Setup

```
sudo -iu postgres createdb demo
cargo install diesel_cli --no-default-features --features postgres
~/.cargo/bin/diesel migration run
```

### Running

```
cargo run
```

Then, open http://localhost:3000/graphql in a web browser.

### Teardown

```
sudo -iu postgres dropdb demo
```