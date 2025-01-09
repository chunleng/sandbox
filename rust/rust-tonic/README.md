# Rust Tonic

This sandbox project is to test on how to use Tonic GRPC server.

## Status

Working

## Getting Started

```bash
# To start the service
cargo run

# To test
grpcurl -proto proto/coffee_service.proto -plaintext localhost:3000 coffee.Coffeeshop/BuyCoffee
ORDER_ID=0; grpcurl -proto proto/coffee_service.proto -plaintext -d "{\"order_id\": \"${ORDER_ID}\"}" localhost:3000 coffee.Coffeeshop/CheckCoffee
```

## Notes

- Currently, only the server is build. I might want to consider building the
  client as well.
