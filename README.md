# Number Rumba Roulette

A web application that allows multiple players to connect and see the same randomized Rumba so that they can solve it in competition.

## Running

```
RUST_LOG=info cargo run
```

### Linux
Configure Server IP in `main.rs`

```
sudo setcap CAP_NET_BIND_SERVICE=+eip target/debug/number-rumba-roulette
```

