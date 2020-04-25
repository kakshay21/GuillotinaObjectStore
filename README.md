# gobject (Guillotina Object Server)

## Setting up
```
brew install protobuf
cargo install grpcio-compiler
cargo install protobuf-codegen
```

## To Sync server with protobuffer
```
protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` transaction.proto
```

## To Sync client with protobuffer
```
python -m grpc_tools.protoc -I. --python_out=client/guillotina_grpc --grpc_python_out=client/guillotina_grpc transaction.proto
```
