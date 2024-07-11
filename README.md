# Rust X gRPC X Unix Socket X socket activation X Snap

Simple PoC to get a socket activated Rust gRPC sever, serving a unix socket, packaged as a Snap

## Run the Rust server

```
cargo run
```

## Hit the server on its unix socket

```
grpcurl -authority "dummy" -plaintext -unix -proto helloworld.proto -d '{"name": "test"}' /tmp/helloworld.sock helloworld.Greeter/SayHello
{
  "message": "Hello test!"
}
```

(You need to include a placeholder authority, see: https://github.com/hyperium/h2/pull/487)
