Websocket server and client app using [ws-rs](https://github.com/housleyjk/ws-rs/), with binary message serialization/deserialization with [deku](https://github.com/sharksforarms/deku/).

## Usage
__running the server__

run the server binary with the `--host` and `--shares` flags
```
$ cargo r --bin server -- --host 127.0.0.1:3012 -s shares
```

__running the client__

run the client binary with the `--host` flag
```
$ cargo r --bin client -- --host ws://127.0.0.1:3012
```

