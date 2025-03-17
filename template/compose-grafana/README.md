# Compose Grafana

This is a template for setting up Grafana server locally for development
purpose. Note that this is not meant for production setup.

## Status

Working

## Getting Started

```bash
docker-compose up

curl -i -X POST http://localhost:4318/v1/metrics -H "Content-Type: application/json" -d @sample-data/metrics.json
curl -i -X POST http://localhost:4318/v1/traces -H "Content-Type: application/json" -d @sample-data/traces.json
curl -i -X POST http://localhost:4318/v1/logs -H "Content-Type: application/json" -d @sample-data/logs.json
```

- Alloy has been set up to show in stdout when Open Telemetry data has been
  received. You should be able to see in the docker log.
- The following URLs are useful UI that we can use:
  * Grafana UI: <http://localhost:3000>
  * Alloy debug console: <http://localhost:12345>

## Sample Data

The `./sample-data` folder's data is downloaded from
[here](https://github.com/open-telemetry/opentelemetry-proto/blob/main/examples/README.md)

## Note

- Loki can only accept logs that happened in the past 7 days. You may need to
  edit the sample JSON file if you want to test sending to Loki.
- When exploring logs on Grafana, remember to filter by labels, else, nothing
  will show.

## Future Work

- To explore how to log traces using [Grafana
  Tempo](https://grafana.com/docs/tempo/latest/)
- To explore how to log metrics using [Prometheus](https://prometheus.io/)
