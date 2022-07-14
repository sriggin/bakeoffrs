# BakeoffRS

## Inspiration

* [Getting started building a gRPC server](https://betterprogramming.pub/building-a-grpc-server-with-rust-be2c52f0860e)

## Setup
```shell
brew install protobuf
brew install grpc
```

## Call the server
```shell
cargo run

# and then ...

grpc_cli call localhost:50051 ivt.TrafficScanner.IsTrafficValid "ip: '1.1.1.1', userAgent: 'this.user.agent'"
```