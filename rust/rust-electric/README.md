# Rust Electric

A sandbox project to test [electric SQL](https://electric-sql.com/).

## Status

Working

## Getting Started

```bash
# Start up the resource necessary to run
docker-compose up

# Run the App
cargo run
```

## Notes

- This app directly uses ElectricSQL but if we are building a webpage, the
  backend server is responsible for talking to ElectricSQL and push changes to
  the client.
- The app sends the changes to directly to Postgres and fetches from
  ElectricSQL. In actual offline app, we probably have to deal with local
  caching of the changes so that the local app can make changes while offline
  and sync the update after going online again. This is not done here as this
  sandbox project aims to implement the sync server only.
