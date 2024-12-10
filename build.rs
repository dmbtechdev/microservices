
// microservices in python, java, go, JS, rust

// Rust
// scalability
// reliabilty
// easy deployment
// flexibility

// development speed is low, low level language
// performance
// safety

// gRPC - remote procedural call (a prottocol like http)
// modern version is gRPC
// uses protobuf - Protocol Buffers
// Protobuf is used to serializing data between programs, like xml or json but rpc is better
// gRPC uses protobuf to define a schema for communication and then the schema is compiled into native code in variaous languages including Rust
// 1. protobuf schema defining the API
// 2. a server which implements the API
// 3. a client to consume said API

// mainly default between programs/processes
// commonly used in a shared network
// much more efficient and performant then others

// alternatives to RPC, (a bit more complex compared to rpc)
// 1. message queues (Amazon SQS)
// 2. publisher-subscriber (PubSub) (Apacahe Kafka)
// 3. RestApi, websockets, etc

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building...");
    tonic_build::compile_protos("proto/authentication.proto")?;
    Ok(())
}