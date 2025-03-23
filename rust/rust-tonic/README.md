# Rust Tonic

This sandbox project is to test on how to use Tonic GRPC server.

## Status

Working

## Coffeeshop Example

The coffeeshop program is a simple program that allows you to

- Order a coffee (A normal request-response gRPC)
- Wait for your order to be ready (A request that has a streaming response)

```bash
# To start the service
cargo run --example=coffeeshop-server

# To test
grpcurl -proto proto/coffee_service.proto -plaintext localhost:3000 coffee.Coffeeshop/BuyCoffee
ORDER_ID=0; grpcurl -proto proto/coffee_service.proto -plaintext -d "{\"order_id\": \"${ORDER_ID}\"}" localhost:3000 coffee.Coffeeshop/CheckCoffee
```

## Chatbot Example

The chatbot program allows you to chat with the bot using a single function
(bidirectional streaming)

```bash
# To start the service
cargo run --example=chatbot-server

# To start the client
cargo run --example=chatbot-client
```
